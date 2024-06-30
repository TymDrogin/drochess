# Rusty Chess Engine

Rusty Chess Engine is a simple chess engine I am working on in my free time.
So far not many things are implemented, howewer the journey is quite enjoyable.

If you are intrested in contributing, please do it. The design of the engine is kept simple and logical.
At the gamestate you will find most of the important structures, such as gamestate, board, castling rights, move defenition etc.
Movegen folder has exactly what you whould expect, utils has some small files that solves some important, but not really main problems.

## Features

- **Game States**: Full implementation of game states.
- **FEN Processor**: Parse and process FEN strings to initialize the board state.
- **Display**: Nicely formatted board and moves display in the console.
- **Movegen**: Bitboard based movegen. I am using rayon whenever it is possible, but performance of it is yet to be tested.

## Libraries 

- **Rayon**: Easy to use parallelization framework that is used a lot in the move generation.
- **Thiserror**: Easy errors.
- **Rand**: Self-explanatory.
- **lazy_static**: A macro for defining lazily evaluated statics.


## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) - Make sure you have Rust installed on your system.

### Installation

1. Clone the repository:
    ```sh
    git clone https://github.com/TymDrogin/rusty-chess-engine/
    ```
2. Navigate to the project directory:
    ```sh
    cd rusty-chess-engine
    ```
3. Build the project:
    ```sh
    cargo build
    ```

### Running the Engine

After building the project, you can run the chess engine using:
```sh
cargo run
```

Unit test can be run using:
```sh
cargo test
```

Becnhmarks are not yet implemented, howewer they are planed.
