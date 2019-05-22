QT += widgets core
QMAKE_CXXFLAGS = -std=c++17
QMAKE_LFLAGS = -std=c++17

SOURCES += \
    src/main.cpp \
    src/Router.cpp \
    src/Robot.cpp \
    src/RobotsHandler.cpp \
    src/mainwindow.cpp

HEADERS += \
    src/Router.hpp \
    src/Robot.hpp \
    src/RobotsHandler.hpp \
    src/mainwindow.h \
    src/tester.h

DISTFILES +=

RESOURCES += \
	ressources.qrc

DEFINES += \
	"ROBOT_APP=\\\"$$_PRO_FILE_PWD_/../robot/target/debug/robot\\\"" \
	"RUST_PARAMS=\\\"RUST_LOG=robot=trace\\\""
