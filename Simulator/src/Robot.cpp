#include "Robot.hpp"

#include <iostream>
#include <sstream>

// To use mkfifo and open/write/close
#include <fcntl.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <unistd.h>

#include <pthread.h>
#include <sched.h>

Robot::Robot(const std::string &fifoName, unsigned int id) : m_id(id)
{
    const char* fifo = fifoName.c_str();
    //m_thread = std::thread(&Robot::instanciate, this, fifoName);

    m_threadArgs.obj = this;
    m_threadArgs.fifoName = fifoName;

    std::cerr << "[ Robot " << id << " ] " << "Opening fifo" << std::endl;
    mkfifo(fifo, 0666);
    pthread_create(&m_thread, nullptr, &Robot::instanciate, (void*)&m_threadArgs);

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

void* Robot::instanciate(void *arguments)
{
    threadArg *args = (threadArg*) arguments;
    //std::this_thread::sleep_for(std::chrono::milliseconds(10));

    struct sched_param params;
    params.sched_priority = -1;
    pthread_setschedparam(args->obj->m_thread, SCHED_FIFO, &params);
    std::stringstream command;
    command << RUST_PARAMS << " " << ROBOT_APP << " -i " << args->fifoName.c_str() << " -o " << args->obj->m_simulFifo.c_str() << " --name " << args->obj->m_id;
    std::cerr << "Execute command " << command.str() << std::endl;
    system(command.str().c_str());

    return args;
}

void Robot::setSimulationOutFifo(const std::string &simulFifo)
{
    m_simulFifo = simulFifo;
}
