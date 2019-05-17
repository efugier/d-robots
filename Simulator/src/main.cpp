#include <QApplication>

#include "Router.hpp"
#include "RobotsHandler.hpp"

int main(int argc, char** argv)
{
    QApplication app(argc, argv);
    QApplication::setAttribute(Qt::AA_UseHighDpiPixmaps);

    app.setOrganizationName("SR05-project");
    app.setApplicationName("Simulateur");

    std::shared_ptr<RobotsHandler> robotList(new RobotsHandler);
    robotList->createRobot("Robot 1", "robot1");
    robotList->createRobot("Robot 2", "robot2");

    Router router(robotList);

    router.listen("simulIn");

    return 0;
}
