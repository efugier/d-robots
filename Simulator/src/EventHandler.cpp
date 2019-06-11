#include "EventHandler.hpp"
#include <QKeyEvent>
#include "RobotsHandler.hpp"
#include <iostream>

EventHandler::EventHandler()
{

}

bool EventHandler::eventFilter(QObject* obj, QEvent* event)
{
    if (event->type() == QEvent::KeyPress) {
           QKeyEvent* key = static_cast<QKeyEvent*>(event);
           switch (key->key())
           {
           case Qt::Key_C:
               emit(newRobot());
               break;
           case Qt::Key_N:
               emit(selectNextRobot());
               break;
           case Qt::Key_P:
               emit(selectPreviousRobot());
               break;
           case Qt::Key_A:
               emit(toggleActive());
               break;
           default:
               return QObject::eventFilter(obj, event);
           }
           return true;
       } else {
           return QObject::eventFilter(obj, event);
       }
       return false;
}
