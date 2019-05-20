#include "Router.hpp"
#include <iostream>
#include <cmath>

#include <QVector2D>

#include <QJsonDocument>
#include <QJsonObject>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

Router::Router(std::shared_ptr<RobotsHandler> robotList, QObject* parent) :
    QObject (parent), m_robotList(robotList)
{

}

std::string Router::cRead(int fd)
{
    char c;
    std::string ret = "";
    do {
        if (read(fd, &c, sizeof(char)) > 0)
        {
            if (c != '\n')
                ret.push_back(c);
        }
        if (!m_listen)
            return {};
    } while (c != '\n');
    return ret;
}

double distance(const QVector2D& pos1, const QVector2D& pos2)
{
    return std::sqrt(std::pow(pos1.x() - pos2.x(), 2) + std::pow(pos1.y() - pos2.y(), 2));
}

void Router::listen(const std::string &fifoName)
{
    const char* fifo = fifoName.c_str();

    mkfifo(fifo, 0666);

    m_fifoFd = open(fifo, O_RDONLY);

    if (m_fifoFd < 0)
    {
        std::cerr << "[ Router ] " << "Fifo error : " <<  strerror(m_fifoFd) << std::endl;
        return;
    }
    std::cerr << "[ Router ] " << "Fifo openned" << std::endl;
    while (true)
    {
        std::string message = cRead(m_fifoFd);
        if (!m_listen)
            break;
        std::cerr << "Received message : " << message << std::endl;

        QJsonDocument messageJson = QJsonDocument::fromJson(QByteArray(message.c_str()));

        if (!messageJson.isObject())
        {
            std::cerr << "The message is not an Object" << std::endl;
            continue;
        }
        auto jsonObj = messageJson.object();
        // { "pos":{"x":1,"y":2},"id":1}

        // Extract position
        if (!jsonObj.contains("pos") || !jsonObj["pos"].isObject())
        {
            std::cerr << "There is not a 'pos' field in the message" << std::endl;
            continue;
        }
        auto jsonPos = jsonObj["pos"].toObject();
        if (!jsonPos.contains("x") || !jsonPos.contains("y") || !jsonPos["x"].isDouble() || !jsonPos["y"].isDouble())
        {
            std::cerr << "Field 'pos' does not contains doubles x and/or y" << std::endl;
            continue;
        }
        QVector2D pos(jsonPos["x"].toDouble(),jsonPos["x"].toDouble());

        // Extract identifiant
        if (!jsonObj.contains("id") || !jsonObj["id"].isDouble())
        {
            std::cerr << "There is not a 'id' field in the message" << std::endl;
        }
        std::string name = "Robot " + std::to_string(jsonObj["id"].toInt());

        if (m_robotList)
        {
            if (Robot* sender = m_robotList->getRobot(name))
            {
                sender->setPosition(pos);
                emit(updateRobotPosition(name));

                float range = sender->range();
                for (auto& it : *m_robotList)
                {
                    if (it.first != name)
                    {
                        if (distance(sender->position(), it.second.position()) < range)
                        {
                            it.second << message;
                        }
                    }
                }
            }
        }
    }


    close(m_fifoFd);

}

void Router::stop()
{
    m_listen = false;
}
