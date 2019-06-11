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
    void selectNextRobot();
    void selectPreviousRobot();
    void toggleActive();

protected:
    bool eventFilter(QObject* obj, QEvent* event) override;
};

