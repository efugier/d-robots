#include "Router.hpp"
#include <iostream>
#include <cmath>

#include <QVector2D>
#include <QTextStream>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

Router::Router(std::shared_ptr<RobotsHandler> robotList) :
    m_robotList(robotList)
{

}

std::string cRead(int fd)
{
    char c;
    std::string ret = "";
    do {
        if (read(fd, &c, sizeof(char)) > 0)
        {
            if (c != '\n')
                ret.push_back(c);
        }
    } while (c != '\n');
    return ret;
}

double distance(const QVector2D& pos1, const QVector2D& pos2)
{
    return std::sqrt(std::pow(pos1.x() - pos2.x(), 2) + std::pow(pos1.y() - pos2.y(), 2));
}

void Router::listen(const QString &fifoName)
{
    const char* fifo = fifoName.toStdString().c_str();

    mkfifo(fifo, 0666);

    m_fifoFd = open(fifo, O_RDONLY);

    if (m_fifoFd < 0)
    {
        std::cerr << "[ Router ] " << "Fifo error : " <<  strerror(m_fifoFd) << std::endl;
        return;
    }
    std::cerr << "[ Router ] " << "Fifo openned" << std::endl;
    while (m_listen)
    {
        std::string message = cRead(m_fifoFd);
        std::cerr << "Received message : " << message << std::endl;

        /*
         * TODO : Extraction position et identifiant
         */

        QVector2D pos(0,0);
        QString name = "Robot 1";

        if (m_robotList)
        {
            if (const Robot* sender = m_robotList->getRobot(name))
            {
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
