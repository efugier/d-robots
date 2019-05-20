#include "Robot.hpp"

#include <iostream>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

Robot::Robot(const std::string &fifoName, const std::string& name) : m_name(name)
{
    const char* fifo = fifoName.c_str();

    std::cerr << "[" << m_name << "] " << "Opening fifo" << std::endl;
    mkfifo(fifo, 0666);

    m_fifoFd = open(fifo, O_WRONLY);

    if (m_fifoFd < 0)
        std::cerr << "[" << m_name << "] " << "Fifo error : " <<  strerror(m_fifoFd) << std::endl;
    std::cerr << "[" << m_name << "] " << "Fifo openned" << std::endl;
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
        write(m_fifoFd, (message + '\n').c_str(), message.size() + 1);
    else
        std::cerr << "[" << m_name << "] " << "Fifo not opened" << std::endl;
}
