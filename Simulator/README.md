# Features
- Manage and show the position of the robots
- Simulate a dynamic network based on the robots ranges

# Build
The robot app must be build before. Execute the following command in the directory containing the binary (usualy target/release).
```
export ROBOT_LOCATION=`pwd`/robot
```

We use Qt, so you need to run `qmake` before:

```
mkdir build
cd build
qmake ../Simulator.pro "ROBOT_APP=\\\"${ROBOT_LOCATION}\\\""
make
```

# Run
To run, execute `./Simulator`.
Here is the key binding:
- C : Create a new robot
- N : Select the next robot
- P : Select the previous robot
- A : Enable or disable the communications of the selected robot

# Files
`main.cpp` : Launch the application and connect the Qt signals and slots
## GUI
`mainwindow.*` : Manage the view and printing actions
`EventHandler.*` : Emit corresponding signals on key strike

## Robots
`Robot.*` : Group all robot informations
`RobotsHandler.*` : List of robots, and robot creation methods

## Router
`Router.*` : Listen, parse and transmit the messages ; update robot positions
`

