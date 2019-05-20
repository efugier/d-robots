#include <iostream>
#include <QApplication>
#include "Router.hpp"
#include "RobotsHandler.hpp"
#include <unistd.h>
#include <stdio.h>
#include <sys/types.h>
#include "mainwindow.h"
#include "tester.h"

int main(int argc, char** argv)
{
    QApplication app(argc, argv);
    QApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    app.setOrganizationName("SR05-project");
    app.setApplicationName("Simulateur");
    MainWindow *w = new MainWindow();
    w->show();
    // Generates random robots and objects for testing puroposes
    Tester t(w, 3000);

    std::shared_ptr<RobotsHandler> robotList(new RobotsHandler);
    robotList->createRobotAsync("Robot 1", "robot1");
    robotList->createRobotAsync("Robot 2", "robot2");

    Router router(robotList);

    std::thread listener(&Router::listen,&router,"simulIn");

    int ret = app.exec();

    router.stop();
    listener.join();
    return ret;
}
