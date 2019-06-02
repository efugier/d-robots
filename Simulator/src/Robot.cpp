#include "Robot.hpp"

#include <iostream>
#include <sstream>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

Robot::Robot(const std::string &fifoName, unsigned int id) : m_id(id)
{
    const char* fifo = fifoName.c_str();

    m_thread = std::thread(&Robot::instanciate, this, fifoName);

    std::cerr << "[ Robot " << id << " ] " << "Opening fifo" << std::endl;
    mkfifo(fifo, 0666);

    m_fifoFd = open(fifo, O_WRONLY);

    if (m_fifoFd < 0)
        std::cerr << "[ Robot " << id << " ] " << "Fifo error : " <<  strerror(m_fifoFd) << std::endl;
    std::cerr << "[ " << id << " ] " << "Fifo openned" << std::endl;
}

Robot::~Robot()
{
    if (m_fifoFd > 0)
        close(m_fifoFd);

}

Robot::Robot(Robot &&move) :
    m_fifoFd(move.m_fifoFd), m_thread(std::move(move.m_thread)), m_pixmap(move.m_pixmap),
    m_range(move.m_range), m_position(std::move(move.m_position))
{
    move.m_fifoFd = 0;
    move.m_pixmap = nullptr;
}

void Robot::operator<<(const std::string &message)
{
    if (m_fifoFd > 0)
        write(m_fifoFd, (message + '\n').c_str(), message.size() + 1);
    else
        std::cerr << "[ Robot " << m_id << " ] " << "Fifo not opened" << std::endl;
}

void Robot::instanciate(const std::string& fifoName)
{
    std::this_thread::sleep_for(std::chrono::seconds(1));
    std::stringstream command;
    command << RUST_PARAMS << " " << ROBOT_APP << " -i " << fifoName.c_str() << " -o " << m_simulFifo.c_str() << " --name " << m_id;
    std::cerr << "Execute command " << command.str() << std::endl;
    system(command.str().c_str());
}

void Robot::setSimulationOutFifo(const std::string &simulFifo)
{
    m_simulFifo = simulFifo;
}
