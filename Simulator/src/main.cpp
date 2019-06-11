#include <iostream>
#include <QApplication>
#include <memory>
#include "Router.hpp"
#include "RobotsHandler.hpp"
#include <unistd.h>
#include <stdio.h>
#include <sys/types.h>
#include <QObject>
#include "mainwindow.h"
#include "tester.h"
#include "EventHandler.hpp"

int main(int argc, char** argv)
{
    std::cout << ROBOT_APP << std::endl;
    std::shared_ptr<RobotsHandler> robotList(new RobotsHandler);

    QApplication app(argc, argv);
    QApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    app.setOrganizationName("SR05-project");
    app.setApplicationName("Simulateur");

    EventHandler eventHandler;

    MainWindow *w = new MainWindow(robotList);
    w->show();

    w->installEventFilter(&eventHandler);
    QObject::connect(&eventHandler, SIGNAL(newRobot()), w, SLOT(createNewRobot()));

    // Generates random robots and objects for testing puroposes
    //Tester t(w, 3000);
    //Robot::setSimulationOutFifo("simulIn");


    std::shared_ptr<Router> router = Router::create(robotList);

    QObject::connect(router.get(), SIGNAL(updateRobotPosition(unsigned int)), w, SLOT(updateRobotPosition(unsigned int)));

//    std::thread listener(&Router::listen,&router,"simulIn");
    Router::newListener("simulIn");


    int ret = app.exec();

    router->stop();
//    listener.join();
    return ret;
}
