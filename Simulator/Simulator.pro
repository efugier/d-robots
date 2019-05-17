QT += widgets core
QMAKE_CXXFLAGS = -std=c++17
QMAKE_LFLAGS = -std=c++17

SOURCES += \
    src/main.cpp \
    src/Router.cpp \
    src/Robot.cpp \
    src/RobotsHandler.cpp

HEADERS += \
    src/Router.hpp \
    src/Robot.hpp \
    src/RobotsHandler.hpp
