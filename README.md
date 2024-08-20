# Arcade29-RustPong

---

## Project Overview

This is a simple Pong game developed in Rust using SDL2 for graphics rendering. The game simulates a classic Pong experience, where two players control rackets to bounce a ball back and forth. The objective is to score points by making the ball pass the opponent's racket. The game includes basic functionalities like collision detection, speed adjustments, and game state management.

## Features

- **Ball Movement:** The ball moves in different directions and speeds based on collisions.
- **Racket Movement:** The rackets can move up and down to hit the ball.
- **Collision Detection:** Detects collisions between the ball and rackets, as well as the screen edges (floor and ceiling).
- **Speed Adjustment:** The ball speed increases as the game progresses, up to a maximum speed.

## Installation

To run this project, you need to have Rust and SDL2 installed on your system.

### Prerequisites

- **Rust**: Install Rust by following the instructions at [Rust's official site](https://www.rust-lang.org/tools/install).
- **SDL2**: 
  - On **Windows**: You can download the development libraries from [SDL2's official site](https://www.libsdl.org/download-2.0.php).
  - On **macOS**: Use `brew` to install SDL2:
    ```bash
    brew install sdl2
    ```
  - On **Linux** (Debian-based systems):
    ```bash
    sudo apt-get install libsdl2-dev
    ```

### Cloning the Repository

Clone the repository to your local machine:

```bash
git clone https://github.com/Unchanted/RustPong
cd RustPong
```

### Building the project

Use cargo, rust's package manager, to build it:
```bash
cargo build --release
```

### Running the game

After building the project, you can run the game using the following command:
```bash
cargo run --release
```
```
