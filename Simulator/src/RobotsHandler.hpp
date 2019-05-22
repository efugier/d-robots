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

    Robot* createRobot(unsigned int id, std::string fifoName = "");
    void createRobotAsync(unsigned int id, std::string fifoName = "", std::function<void(Robot*)> callback = {});

    const Robot* getRobot(unsigned int id) const;
    Robot* getRobot(unsigned int id);

    std::map<unsigned int, Robot>::iterator begin() { return m_robotList.begin(); }
    std::map<unsigned int, Robot>::iterator end() { return m_robotList.end(); }
private:
    void createRobotAsyncThread(unsigned int id, std::string fifoName = "", std::function<void(Robot*)> callback = {});
    std::map<unsigned int, Robot> m_robotList;

    std::vector<std::shared_ptr<std::thread>> m_threadList;

};

