# Drochess Chess Engine

**Drochess** is a chess engine I'm developing in my free time.  
The engine is fully implemented in Rust and aims to be both **simple in design** and **fast** in execution.

I'm using most conventional techniques for chess engine development while maintaining **clarity** in design and leveraging **Rust’s developer-friendly features** wherever possible.

If you're interested in contributing, feel free to jump in!  
Each module does exactly what you'd expect it to do. Utility functions that don’t fit a specific category are located in the `utils` module.

---

## Features

- **Game States** – Full implementation of game state management.
- **FEN Processor** – Parses and processes FEN strings to initialize the board.
- **Display** – Nicely formatted board and move display in the console.
- **Move Generation** – Bitboard-based move generation.

---

## Libraries Used

- [`thiserror`](https://crates.io/crates/thiserror) – For simple and ergonomic error handling.
- [`rand`](https://crates.io/crates/rand) – Random number generation (e.g., Zobrist hashing).
- [`once_cell`](https://crates.io/crates/once_cell) – Used to define compile-time arrays of random numbers for Zobrist hashing.

---

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) – Install Rust with `rustup`.
- A **CPU with BMI (Bit Manipulation Instructions)** support is currently required. (for PEXT-based sliding pieces movegen).
- Future versions will include **magic bitboards** as a fallback for multiplatform support.

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

To run the chess engine:
```sh
cargo run
