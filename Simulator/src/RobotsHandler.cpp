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

Robot* RobotsHandler::createRobot(const std::string &name, std::string fifoName)
{
    std::cerr << "[" << name << "] " << "Creating robot" << std::endl;
    if (m_robotList.count(name) != 0)
        return &m_robotList.at(name);

    if (fifoName == "")
        fifoName = name;
    std::replace(fifoName.begin(), fifoName.end(), ' ', '_');

    m_robotList.emplace(std::pair<std::string, Robot>(name, Robot(fifoName, name)));

    return &m_robotList.at(name);
}

void RobotsHandler::createRobotAsync(const std::string &name, std::string fifoName, std::function<void (Robot *)> callback)
{
    m_threadList.push_back(std::shared_ptr<std::thread>(new std::thread(&RobotsHandler::createRobotAsyncThread, this, name, fifoName, callback)));
}

const Robot *RobotsHandler::getRobot(const std::string &name) const
{
    if (m_robotList.count(name) == 0)
        return nullptr;

    return &m_robotList.at(name);
}

Robot *RobotsHandler::getRobot(const std::string &name)
{
    if (m_robotList.count(name) == 0)
        return nullptr;

    return &m_robotList.at(name);

}

void RobotsHandler::createRobotAsyncThread(const std::string &name, std::string fifoName, std::function<void (Robot *)> callback)
{
    std::cerr << "[" << name << "] " << "Starting thread" << std::endl;
    if (callback)
        callback(createRobot(name, fifoName));
    else
        createRobot(name, fifoName);
}
