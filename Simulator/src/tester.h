#ifndef TESTER_H
#define TESTER_H

#include "mainwindow.h"
#include <QObject>
#include <QTimer>
#include <cstdlib>
#include "mainwindow.h"
#include <iostream>

/** \class Tester
 * \brief Simulation tester
 *
 * Generates or moves a random robot object periodically
 */

class Tester : public QObject
{
    Q_OBJECT

    QTimer timer;
    int counter;
    int period;
    MainWindow *main_w;

private slots:
    void update(){
        int choice = rand()%2;
        int x = rand()%1000;
        int y = rand()%1000;
        if(choice == 0){
            main_w->addObject(x,y);
            std::cerr << "Object added at " << x << "," << y << std::endl;
        }
        else{
            //main_w->updateRobotPosition("Robot"+std::to_string(counter),x,y);
            std::cerr << "Robot added at " << x << "," << y << std::endl;
            counter = counter == 4 ? 0 : counter+1;
        }
        timer.start(period);
    }
public:
    explicit Tester(MainWindow *main_window, int time_period){
        counter = 0;
        period = time_period;
        main_w = main_window;
        connect(&timer, SIGNAL(timeout()), this, SLOT(update()));
        timer.start(period);
    }
};
#endif // TESTER_H
