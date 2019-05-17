#include "RobotsHandler.hpp"
#include "Robot.hpp"

RobotsHandler::RobotsHandler()
{

}

Robot* RobotsHandler::createRobot(const QString &name, QString fifoName)
{
    if (m_robotList.count(name) != 0)
        return &m_robotList.at(name);

    if (fifoName == "")
        fifoName = name;
    std::replace(fifoName.begin(), fifoName.end(), ' ', '_');

    m_robotList.emplace(std::pair<QString, Robot>(name, Robot(fifoName, name)));

    return &m_robotList.at(name);
}

const Robot *RobotsHandler::getRobot(const QString &name) const
{
    if (m_robotList.count(name) == 0)
        return nullptr;

    return &m_robotList.at(name);
}

Robot *RobotsHandler::getRobot(const QString &name)
{
    if (m_robotList.count(name) == 0)
        return nullptr;

    return &m_robotList.at(name);

}
