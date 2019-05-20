#pragma once
#include <memory>

#include <QString>

#include "RobotsHandler.hpp"

class Router
{
public:
    Router(std::shared_ptr<RobotsHandler> robotList);

    void listen(const QString& fifoName);

    void stop();

private:
    std::string cRead(int fd);

    std::shared_ptr<RobotsHandler> m_robotList;
    int m_fifoFd = 0;
    bool m_listen = true;
};

