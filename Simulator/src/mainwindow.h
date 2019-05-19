#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <unordered_map>
#include <QMainWindow>
#include <QGraphicsScene>
#include <QGraphicsView>
#include <QGraphicsItem>

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
    std::unordered_map<std::string,QGraphicsPixmapItem*> robots;
    QPixmap *robotPm;

public:
    explicit MainWindow(QWidget *parent = nullptr);
    void updateRobotPosition(std::string id, double x, double y);
    void addObject(double x, double y);

signals:

};

#endif // MAINWINDOW_H
