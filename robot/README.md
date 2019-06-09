An app that does app things
===

## Features

* fowards messages on the first reception
* can interact with an asynchronous robot simulator / interface

The script `cargo_run.sh` can be used to launch the project, note that it requires a fifo named `fifo` to run. 
* `./cargo_run.sh` connects the app to itself and enables maximum debug level
* `./cargo_run.sh -- release` connects the app to itself in release mode

* `./launch.py 3 info`  creates 3 instances of the app in 3 terminals with log-level info (`trace > debug > info > warn > error`)
* `./launch.py 3 error --release`  creates 3 instances of the app in release mode, every argument after the log level is fowarded to cargo

## File structure

```
src
├── main.rs
├── app.rs
├── events.rs
├── messages.rs
├── ai
│  └── mod.rs
└── robot
   └── mod.rs
```

### `app.rs`

The organizer, distributes events and tasks between the different components.

### `events.rs`

Implements a struct that listen asynchronously to the different input sources of an app : the other apps (through a pipe network), the robot and the app itself.

### `messages.rs`

Contains the code relative to the messages send over the network.

### `ai/*.rs`

Here are stored all the files relative to the distributed mapping algorithms.

### `robot/*.rs`

This folder holds the robot simulator when the project is compiled in debug mode and the robot interface when compiled in release mode.


## Todo

- [ ] add todos


### Optional

- [ ] move from random ids to sequence numbers
- [ ] add vector clock (ready, just not sure if necessary)
- [ ] actual robot interface
