#pragma once

#include <fstream>

#include <QString>
#include <QVector2D>

#define FIFO_PERMISSION 0666

class Robot
{
public:
    Robot(const QString& fifoName, const QString &name);
    ~Robot();

    Robot(Robot&& move);

    void operator<<(const std::string &message);

    float range() const { return m_range; }
    void setRange(float range) { m_range = range; }

    QVector2D position() const {return m_position; }
    void setPosition(const QVector2D& position) { m_position = position;}

private:
    QString m_name;
    int m_fifoFd = 0;

    float m_range = 10;
    QVector2D m_position;
};

