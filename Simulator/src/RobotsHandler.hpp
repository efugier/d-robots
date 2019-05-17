#pragma once

#include <map>

#include "Robot.hpp"

class Robot;

class RobotsHandler
{
public:
    RobotsHandler();

    Robot* createRobot(const QString& name, QString fifoName = "");

    const Robot* getRobot(const QString& name) const;
    Robot* getRobot(const QString& name);

    std::map<const QString, Robot>::iterator begin() { return m_robotList.begin(); }
    std::map<const QString, Robot>::iterator end() { return m_robotList.end(); }
private:
    std::map<const QString, Robot> m_robotList;

};

