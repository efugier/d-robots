An app that does app things
===

## Features

* forwards messages on the first reception
* can interact with an asynchronous robot simulator / interface

The script `cargo_run.sh` can be used to launch the project, note that it requires a fifo named `fifo` to run. 
* `./cargo_run.sh` connects the app to itself and enables maximum debug level
* `./cargo_run.sh -- release` connects the app to itself in release mode

* `./launch.py 3 info`  creates 3 instances of the app in 3 terminals with log-level info (`trace > debug > info > warn > error`)
* `./launch.py 3 error --release`  creates 3 instances of the app in release mode, every argument after the log level is forwarded to cargo
* `./output/viewer.html` allows to easily visualize the map as seen by each robot.

## File structure

```
src
├── main.rs
├── events.rs
├── messages.rs
├── map
│  ├── mod.rs
│  └── polygon.rs
├── app.rs
├── ai
│  ├── mod.rs
│  └── pathfinder.rs
└── robot
   └── mod.rs
```


### `events.rs`

Implements a struct that listen asynchronously to the different input sources of an app : the other apps (through a pipe network), the robot and the app itself.

### `messages.rs`

Contains the code relative to the messages send over the network.

### `map/*.rs`

Logic of the real map use for the simulation and some basic algebra.

### `app.rs`

The organizer, distributes events and tasks between the different components. Receiver and share information to other robots.

### `ai/*.rs`

Here are stored all the files relative to the distributed mapping algorithms. Outputs pngs for visualization.

### `robot/*.rs`

This folder holds the robot simulator when the project is compiled in debug mode and the robot interface when compiled in release mode.
