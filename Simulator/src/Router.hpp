#pragma once
#include <memory>

#include <QString>

#include "RobotsHandler.hpp"

class Router
{
public:
    Router(std::shared_ptr<RobotsHandler> robotList);

    void listen(const QString& fifoName);

private:
    std::shared_ptr<RobotsHandler> m_robotList;
    int m_fifoFd = 0;
    bool m_listen = true;
};

