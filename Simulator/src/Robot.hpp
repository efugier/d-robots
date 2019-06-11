#pragma once

#include <fstream>
#include <thread>
#include <pthread.h>

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
    void setPosition(const QVector2D& position) { m_lastPosition = m_position; m_position = position;}
    QVector2D lastPosition() const { return m_lastPosition; }

    unsigned int id() const { return m_id; }

    //void instanciate(const std::string& fifoName);
    static void* instanciate(void *arguments);

    QGraphicsPixmapItem* pixmapItem() const { return m_pixmap; }
    void setPixmapItem(QGraphicsPixmapItem* item) { m_pixmap = item; }

    static void setSimulationOutFifo(const std::string& simulFifo);

private:
    unsigned int m_id;
    //static inline std::string m_simulFifo = "";
    int m_fifoFd = 0;

    //std::thread m_thread;
    pthread_t m_thread;
    pthread_attr_t m_threadAtt;

    typedef struct {
        Robot* obj;
        std::string inFifoName;
        std::string outFifoName;
    } threadArg;

    threadArg m_threadArgs;

    QGraphicsPixmapItem* m_pixmap = nullptr;


    float m_range = 1;
    QVector2D m_position;
    QVector2D m_lastPosition;
};

