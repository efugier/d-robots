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
    const int ITEM_HEIGHT = 10;
    const int ITEM_WIDTH = 10;

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
    void createNewRobot();

    void selectNextRobot();
    void selectPreviousRobot();

    void toggleActive();
protected:
    unsigned int m_lastId = 0;

    std::map<unsigned int, Robot>::iterator m_itrRobot;

    QGraphicsEllipseItem* m_selectionShape;

    void loadMap(const QString &filename);

private:
    static constexpr inline char KEY_POLYGONS[] = "polygons";
    static constexpr inline char KEY_POINTS[] = "points";
    static constexpr inline char KEY_CLOSED[] = "is_closed";
    static constexpr inline char KEY_X[] = "x";
    static constexpr inline char KEY_Y[] = "y";
};

#endif // MAINWINDOW_H
