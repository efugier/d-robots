#pragma once

#include <QObject>
#include <memory>

class RobotHandler;

class EventHandler : public QObject
{
    Q_OBJECT
public:
    EventHandler();

signals:
    void newRobot();

protected:
    bool eventFilter(QObject* obj, QEvent* event) override;
};

