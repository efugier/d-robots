QT += widgets core
QMAKE_CXXFLAGS = -std=c++17
QMAKE_LFLAGS = -std=c++17

SOURCES += \
    src/main.cpp \
    src/Router.cpp \
    src/Robot.cpp \
    src/RobotsHandler.cpp \
    src/mainwindow.cpp \
    src/EventHandler.cpp

HEADERS += \
    src/Router.hpp \
    src/Robot.hpp \
    src/RobotsHandler.hpp \
    src/mainwindow.h \
    src/tester.h \
    src/EventHandler.hpp

DISTFILES +=

RESOURCES += \
	ressources.qrc

isEmpty(ROBOT_APP) {
ROBOT_APP=$$_PRO_FILE_PWD_/../robot/target/debug/robot
}

DEFINES += \
	"ROBOT_APP=\\\"$$ROBOT_APP\\\"" \
    "RUST_PARAMS=\\\"RUST_LOG=error\\\""
