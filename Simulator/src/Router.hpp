#pragma once
#include <memory>

#include <QString>
#include <QObject>

#include "RobotsHandler.hpp"

class Router : public QObject
{
    Q_OBJECT
public:
    Router(std::shared_ptr<RobotsHandler> robotList, QObject* parent = nullptr);

    void listen(const std::string &fifoName);

    void stop();
signals:
    void updateRobotPosition(std::string id);

private:
    std::string cRead(int fd);

    std::shared_ptr<RobotsHandler> m_robotList;
    int m_fifoFd = 0;
    bool m_listen = true;
};

