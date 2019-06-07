#include "EventHandler.hpp"
#include <QKeyEvent>
#include "RobotsHandler.hpp"

EventHandler::EventHandler()
{

}

bool EventHandler::eventFilter(QObject* obj, QEvent* event)
{
    if (event->type()==QEvent::KeyPress) {
           QKeyEvent* key = static_cast<QKeyEvent*>(event);
           if (key->key()==Qt::Key_N) {
               emit(newRobot());
           } else {
               return QObject::eventFilter(obj, event);
           }
           return true;
       } else {
           return QObject::eventFilter(obj, event);
       }
       return false;
}
