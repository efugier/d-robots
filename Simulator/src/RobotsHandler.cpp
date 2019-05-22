#include "RobotsHandler.hpp"
#include "Robot.hpp"
#include <iostream>
#include <QThread>

RobotsHandler::RobotsHandler()
{

}

RobotsHandler::~RobotsHandler()
{
    for (auto& t : m_threadList)
        t->join();
}

Robot* RobotsHandler::createRobot(unsigned int id, std::string fifoName)
{
    std::cerr << "[ Robot " << id << " ] " << "Creating robot" << std::endl;
    if (m_robotList.count(id) != 0)
        return &m_robotList.at(id);

    if (fifoName == "")
        fifoName = std::string("robot") + std::to_string(id);
    std::replace(fifoName.begin(), fifoName.end(), ' ', '_');

    m_robotList.emplace(std::pair<unsigned int, Robot>(id, Robot(fifoName, id)));

    return &m_robotList.at(id);
}

void RobotsHandler::createRobotAsync(unsigned int id, std::string fifoName, std::function<void (Robot *)> callback)
{
    m_threadList.push_back(std::shared_ptr<std::thread>(new std::thread(&RobotsHandler::createRobotAsyncThread, this, id, fifoName, callback)));
}

const Robot *RobotsHandler::getRobot(unsigned int id) const
{
    if (m_robotList.count(id) == 0)
        return nullptr;

    return &m_robotList.at(id);
}

Robot *RobotsHandler::getRobot(unsigned int id)
{
    if (m_robotList.count(id) == 0)
        return nullptr;

    return &m_robotList.at(id);

}

void RobotsHandler::createRobotAsyncThread(unsigned int id, std::string fifoName, std::function<void (Robot *)> callback)
{
    std::cerr << "[" << id << "] " << "Starting thread" << std::endl;
    if (callback)
        callback(createRobot(id, fifoName));
    else
        createRobot(id, fifoName);
}
