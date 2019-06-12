ROOT_DIR:=$(shell dirname $(realpath $(lastword $(MAKEFILE_LIST))))

BUILD_DIR=${ROOT_DIR}/build

SIMULATOR_DIR=${ROOT_DIR}/Simulator
SIMULATOR_PROJECT=Simulator.pro

ROBOT_DIR=${ROOT_DIR}/robot

QMAKE=qmake-qt5
MAKE=make
PROCESSUS=2

CARGO=cargo


all: buildSimu buildRobot

buildSimu: buildDir
	cd ${BUILD_DIR} && \
	${QMAKE} ${SIMULATOR_DIR}/${SIMULATOR_PROJECT} "ROBOT_APP=\\\"${BUILD_DIR}/debug/robot\\\"" && \
	${MAKE} -j${PROCESSUS}

buildRobot: buildDir
	echo "Cargo : ${CARGO}"
	${CARGO} build --release --target-dir ${BUILD_DIR} --manifest-path ${ROBOT_DIR}/Cargo.toml

buildDir:
	mkdir -p ${BUILD_DIR}

clean:
	rm -rf ${BUILD_DIR}
