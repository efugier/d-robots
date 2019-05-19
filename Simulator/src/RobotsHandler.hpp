#pragma once

#include <map>
#include <functional>
#include <memory>
#include <thread>

#include "Robot.hpp"

class Robot;

class RobotsHandler
{
public:
    RobotsHandler();
    ~RobotsHandler();

    Robot* createRobot(const QString& name, QString fifoName = "");
    void createRobotAsync(const QString& name, QString fifoName = "", std::function<void(Robot*)> callback = {});

    const Robot* getRobot(const QString& name) const;
    Robot* getRobot(const QString& name);

    std::map<const QString, Robot>::iterator begin() { return m_robotList.begin(); }
    std::map<const QString, Robot>::iterator end() { return m_robotList.end(); }
private:
    void createRobotAsyncThread(const QString& name, QString fifoName = "", std::function<void(Robot*)> callback = {});
    std::map<const QString, Robot> m_robotList;

    std::vector<std::shared_ptr<std::thread>> m_threadList;

};

