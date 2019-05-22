#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <unordered_map>
#include <QMainWindow>
#include <QGraphicsScene>
#include <QGraphicsView>
#include <QGraphicsItem>

#include "RobotsHandler.hpp"

/** \class MainWindow
 * \brief Simulation window
 *
 * Displays the current map, with robots and objects position
 */



class MainWindow : public QMainWindow
{
    Q_OBJECT

    // Image size for robots and objects
    const int ITEM_HEIGHT = 60;
    const int ITEM_WIDTH = 60;

    // Minimum size of the map view
    const double MIN_SCENE_HEIGHT = 300;
    const double MIN_SCENE_WIDTH = 300;

    QGraphicsScene *scene;
    QGraphicsView *view;
    QPixmap *robotPm;

    std::shared_ptr<RobotsHandler> m_robotList;

public:
    explicit MainWindow(std::shared_ptr<RobotsHandler> robotList, QWidget *parent = nullptr);

public slots:
    void updateRobotPosition(unsigned int id);
    void addObject(double x, double y);


};

#endif // MAINWINDOW_H
