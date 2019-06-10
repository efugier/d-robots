#include "mainwindow.h"
#include <iostream>
#include <QString>
#include <QJsonDocument>
#include <QJsonObject>
#include <QJsonArray>
#include <QKeyEvent>
#include <QColor>

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

    loadMap();
}

void MainWindow::loadMap()
{
    QFile file("map/map.json");
    file.open(QIODevice::ReadOnly);

    QJsonDocument jsonDoc = QJsonDocument::fromJson(file.readAll());


    file.close();

    if (!jsonDoc.isObject())
    {
        std::cerr << "The document following is not an object :" << std::endl;
        std::cerr << jsonDoc.toBinaryData().toStdString() << std::endl;
        return;
    }
    if (!jsonDoc.object().contains(KEY_POLYGONS) || !jsonDoc.object()[KEY_POLYGONS].isArray())
    {
        std::cerr << "The object does not contains an array '" << KEY_POLYGONS << "'" << std::endl;
        return;
    }
    auto polygons = jsonDoc.object()[KEY_POLYGONS].toArray();

    QPen pen((QColor(Qt::blue)));
    for (auto rawPolygon : polygons)
    {
        if (!rawPolygon.isObject())
        {
            std::cerr << "A polygon is not an object" << std::endl;
            return;
        }
        auto polygon = rawPolygon.toObject();
        if (!polygon.contains(KEY_CLOSED) || !polygon[KEY_CLOSED].isBool())
        {
            std::cerr << "The following polygon does not contains the boolean '" << KEY_CLOSED << "'" << std::endl;
            std::cerr << rawPolygon.toString().toStdString() << std::endl;
            return;
        }
        bool closed = polygon[KEY_CLOSED].toBool();

        if (!polygon.contains(KEY_POINTS) || !polygon[KEY_POINTS].isArray())
        {
            std::cerr << "The following polygon does not contains the array '" << KEY_POINTS << "'" << std::endl;
            std::cerr << rawPolygon.toString().toStdString() << std::endl;
            return;
        }

        auto points = polygon[KEY_POINTS].toArray();

        QPoint firstPoint;
        bool firstPointDefined = false;
        QPoint lastPoint;

        for (auto rawPoint : points)
        {
            if (!rawPoint.isObject())
            {
                std::cerr << "The following point is not an object" << std::endl;
                std::cerr << rawPoint.toString().toStdString() << std::endl;
                return;
            }
            auto point = rawPoint.toObject();
            if (!point.contains(KEY_X) || !point.contains(KEY_Y) ||
                    !point[KEY_X].isDouble() || !point[KEY_Y].isDouble())
            {
                std::cerr << "The following point does not contains the doubles '" << KEY_X << "' and/or '" << KEY_Y << "'" << std::endl;
                std::cerr << rawPoint.toString().toStdString() << std::endl;
                return;
            }
            QPoint currentPoint = QPoint{point[KEY_X].toInt() * 10, point[KEY_Y].toInt() * 10};
            if (!firstPointDefined)
            {
                firstPointDefined = true;
                firstPoint = currentPoint;
            }
            else
                scene->addLine(QLine(lastPoint, currentPoint), pen);

            lastPoint = currentPoint;
        }
        if (closed)
        {
            scene->addLine(QLine(lastPoint, firstPoint), pen);
        }
    }
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

void MainWindow::createNewRobot()
{
    if (m_robotList)
    {
        m_robotList->createRobotAsync(++m_lastId, "", [this](Robot* r){
            this->updateRobotPosition(r->id());
        });

    }
}
