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

std::shared_ptr<Router> Router::m_instance;

double distance(const QVector2D& pos1, const QVector2D& pos2)
{
    return std::sqrt(std::pow(pos1.x() - pos2.x(), 2) + std::pow(pos1.y() - pos2.y(), 2));
}

Router::Router(std::shared_ptr<RobotsHandler> robotList, QObject* parent) :
    QObject (parent), m_robotList(robotList)
{

}

std::string Router::cRead(int fd)
{
    constexpr ssize_t BUFFER_SIZE = 1000000;
    char buffer[BUFFER_SIZE];
    std::string ret = "";
    std::string strBuffer = "";
    ssize_t readSize = 0;
    do {
        if ((readSize = read(fd, &buffer, BUFFER_SIZE)) > 0)
        {
            strBuffer = buffer;
            ret.append(strBuffer.substr(0, readSize-1));
            //std::cerr << "Add " << readSize << " length buffer" << std::endl;
        }
        if (!m_listen)
            return {};
    } while (buffer[readSize-1] != '\n' || readSize <= 1);
    //std::cerr << "Last character : [" << readSize-1 << "] '" << buffer[readSize-1] << "'" << std::endl;
    return ret;
}

void Router::worker(std::string message)
{
    QJsonDocument messageJson = QJsonDocument::fromJson(QByteArray(message.c_str()));

    //std::cout << "Message : " << message << std::endl;
    if (!messageJson.isObject())
    {
        std::cerr << "The message is not an Object" << std::endl;
        return;
    }
    auto jsonObj = messageJson.object();
    // { "pos":{"x":1,"y":2},"id":1}

    // Extract position
    if (!jsonObj.contains(KEY_POSITION) || !jsonObj[KEY_POSITION].isObject())
    {
        std::cerr << "There is not a '" << KEY_POSITION << "' field in the message" << std::endl;
        return;
    }
    auto jsonPos = jsonObj[KEY_POSITION].toObject();
    if (!jsonPos.contains(KEY_POINT) || !jsonPos[KEY_POINT].isObject())
    {
        std::cerr << "Field '" << KEY_POSITION << "' does not contains object " << KEY_POINT << std::endl;
        return;
    }
    jsonPos = jsonPos[KEY_POINT].toObject();
    if (!jsonPos.contains(KEY_X) || !jsonPos.contains(KEY_Y) || !jsonPos[KEY_X].isDouble() || !jsonPos[KEY_Y].isDouble())
    {
        std::cerr << "Field '" << KEY_POSITION << "' does not contains doubles " << KEY_X << " and/or " << KEY_Y<< std::endl;
        return;
    }
    QVector2D pos(jsonPos[KEY_X].toDouble(),jsonPos[KEY_Y].toDouble());

    // Extract identifiant
    if (!jsonObj.contains(KEY_SENDER_ID) || !jsonObj[KEY_SENDER_ID].isDouble())
    {
        std::cerr << "There is not a " << KEY_SENDER_ID <<" field in the message" << std::endl;
        return;
    }
    unsigned int id= jsonObj[KEY_SENDER_ID].toInt();


    if (m_robotList)
    {
        if (Robot* sender = m_robotList->getRobot(id))
        {
            if (!sender->active())
                return;

            sender->setPosition(pos);
            emit(updateRobotPosition(id));

            float range = sender->range();
            for (auto& [rId, r] : *m_robotList)
            {
                if (rId != id)
                {
                    if (!r.active())
                        continue;
                    if (distance(sender->position(), r.position()) < range)
                    {
                        std::cerr << "Message from " << id << " transmitted to " << rId << std::endl;
                        r << message;
                    }
                }
            }
        }
    }
}

void Router::newListener(const std::string& fifoName)
{
    if (Router::get())
        std::thread(&Router::listen, Router::get().get(), fifoName).detach();
}


std::shared_ptr<Router> Router::create(std::shared_ptr<RobotsHandler> robotList)
{
    if (!m_instance)
        m_instance = std::shared_ptr<Router>(new Router(robotList));

    return m_instance;
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
        //std::cerr << fifoName << " : Received message : " << message << std::endl;

        std::thread (&Router::worker, this, std::string(message)).detach();

    }


    close(m_fifoFd);

}

void Router::stop()
{
    m_listen = false;
}
