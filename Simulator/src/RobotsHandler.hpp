#pragma once

#include <map>
#include <functional>
#include <memory>
#include <thread>

#include <QObject>

#include "Robot.hpp"

class Robot;

class RobotsHandler
{
public:
    RobotsHandler();
    ~RobotsHandler();

    Robot* createRobot(const std::string& name, std::string fifoName = "");
    void createRobotAsync(const std::string& name, std::string fifoName = "", std::function<void(Robot*)> callback = {});

    const Robot* getRobot(const std::string &name) const;
    Robot* getRobot(const std::string& name);

    std::map<const std::string, Robot>::iterator begin() { return m_robotList.begin(); }
    std::map<const std::string, Robot>::iterator end() { return m_robotList.end(); }
private:
    void createRobotAsyncThread(const std::string& name, std::string fifoName = "", std::function<void(Robot*)> callback = {});
    std::map<const std::string, Robot> m_robotList;

    std::vector<std::shared_ptr<std::thread>> m_threadList;

};

