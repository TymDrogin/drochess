# Drochess chess engine

Drochess is a chess engine I am working on in my free time.
The engine is fully implemented in rust, and aims to be simple in design and fast.
I am trying use most of the conventional tecnics for developing an engine while 
maintaining the clarity of the design and using as many nice developer features rust has to offer as possible.

If you are intrested in contributing, please do it.
Every module does exaclty what you whould expect it to do, and some features that has no defined category are located in utils.

## Features

- **Game States**: Full implementation of game states.
- **FEN Processor**: Parse and process FEN strings to initialize the board state.
- **Display**: Nicely formatted board and moves display in the console.
- **Movegen**: Bitboard based movegen. I am using rayon whenever it is possible, but performance of it is yet to be tested.

## Libraries 

- **Rayon**: Easy to use parallelization framework that is used a lot in the move generation.
- **Thiserror**: Easy errors.
- **Rand**: Self-explanatory.
- **lazy_static**: A macro for defining lazily evaluated statics. Used mostly for lookup tables etc.


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
