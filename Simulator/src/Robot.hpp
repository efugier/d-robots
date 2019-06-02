#pragma once

#include <fstream>
#include <thread>

#include <QGraphicsPixmapItem>

#include <QVector2D>

#define FIFO_PERMISSION 0666

class Robot
{
public:
    Robot(const std::string& fifoName, unsigned int id);
    ~Robot();

    Robot(Robot&& move);

    void operator<<(const std::string &message);

    float range() const { return m_range; }
    void setRange(float range) { m_range = range; }

    QVector2D position() const {return m_position; }
    void setPosition(const QVector2D& position) { m_position = position;}

    unsigned int id() const { return m_id; }

    void instanciate(const std::string& fifoName);

    QGraphicsPixmapItem* pixmapItem() const { return m_pixmap; }
    void setPixmapItem(QGraphicsPixmapItem* item) { m_pixmap = item; }

    static void setSimulationOutFifo(const std::string& simulFifo);

private:
    unsigned int m_id;
    static inline std::string m_simulFifo = "";
    int m_fifoFd = 0;

    std::thread m_thread;

    QGraphicsPixmapItem* m_pixmap = nullptr;


    float m_range = 10;
    QVector2D m_position;
};

