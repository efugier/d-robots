#include "Robot.hpp"

#include <iostream>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

Robot::Robot(const QString &fifoName, const QString& name) : m_name(name)
{
    const char* fifo = fifoName.toStdString().c_str();

    mkfifo(fifo, FIFO_PERMISSION);

    m_fifoFd = open(fifo, O_WRONLY);

    if (m_fifoFd < 0)
        std::cerr << "[" << m_name.toStdString() << "] " << "Fifo error : " <<  strerror(m_fifoFd) << std::endl;

    *this << "Bonjour";
}

Robot::~Robot()
{
    if (m_fifoFd > 0)
        close(m_fifoFd);

}

Robot::Robot(Robot &&move) : m_name(std::move(move.m_name)), m_fifoFd(move.m_fifoFd)
{
    move.m_fifoFd = 0;
}

void Robot::operator<<(const std::string &message)
{
    if (m_fifoFd > 0)
        write(m_fifoFd, message.c_str(), message.size() + 1);
    else
        std::cerr << "[" << m_name.toStdString() << "] " << "Fifo not opened" << std::endl;
}
