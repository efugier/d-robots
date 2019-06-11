#pragma once
#include <memory>

#include <QString>
#include <QObject>

#include "RobotsHandler.hpp"

class Router : public QObject
{
    Q_OBJECT
public:
    static std::shared_ptr<Router> create(std::shared_ptr<RobotsHandler> robotList);
    static std::shared_ptr<Router> get() { return m_instance; }

    void listen(const std::string &fifoName);

    void stop();
    static void newListener(const std::string& fifoName);
signals:
    void updateRobotPosition(unsigned int id);


private:
    static std::shared_ptr<Router> m_instance;
    Router(std::shared_ptr<RobotsHandler> robotList, QObject* parent = nullptr);
    std::string cRead(int fd);

    void worker(std::string message);

    std::shared_ptr<RobotsHandler> m_robotList;
    int m_fifoFd = 0;
    bool m_listen = true;

    static constexpr inline char KEY_SENDER_ID[] = "sender_id";
    static constexpr inline char KEY_POSITION[] = "pos";
    static constexpr inline char KEY_POINT[] = "p";
    static constexpr inline char KEY_X[] = "x";
    static constexpr inline char KEY_Y[] = "y";
};

