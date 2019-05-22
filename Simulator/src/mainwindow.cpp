#include "mainwindow.h"
#include <iostream>
#include <QString>

MainWindow::MainWindow(std::shared_ptr<RobotsHandler> robotList, QWidget *parent) :
    QMainWindow(parent), m_robotList(robotList)
{
    setMinimumSize(500,500);
    scene = new QGraphicsScene(this);
    view = new QGraphicsView(this);
    setCentralWidget(view);

    view->setScene(scene);

    robotPm = new QPixmap(":/img/robot.png");
    std::cerr << "MainWindow created" << std::endl;
    if(robotPm->isNull()) std::cerr << "Error creating pixmap" << std::endl;
}


// Adds or moves on the map the robot with given id
void MainWindow::updateRobotPosition(unsigned int id)
{
    if (!m_robotList)
        return;
    if (Robot* robot = m_robotList->getRobot(id))
    {
        if (!robot->pixmapItem())
        {
            robot->setPixmapItem(scene->addPixmap(robotPm->scaled(ITEM_WIDTH,ITEM_HEIGHT,Qt::KeepAspectRatio, Qt::FastTransformation)));
            robot->pixmapItem()->setVisible(true);
        }

        robot->pixmapItem()->setX(robot->position().x());
        robot->pixmapItem()->setY(robot->position().y());
        robot->pixmapItem()->setToolTip(QString::fromStdString(id+" : "+std::to_string(robot->position().x())+","+std::to_string(robot->position().y())));
        view->fitInView(scene->sceneRect().x(),
                        scene->sceneRect().y(),
                        std::max(scene->sceneRect().width(),MIN_SCENE_WIDTH),
                        std::max(scene->sceneRect().height(),MIN_SCENE_HEIGHT),
                        Qt::KeepAspectRatio);
    }
}

// Adds an object on the map
void MainWindow::addObject(double x, double y){
    QGraphicsRectItem *rect = scene->addRect(x,y,ITEM_WIDTH,ITEM_HEIGHT);
    rect->setToolTip(QString::fromStdString("Obstacle : "+std::to_string(x)+","+std::to_string(y)));
    view->fitInView(scene->sceneRect(), Qt::KeepAspectRatio);
}
