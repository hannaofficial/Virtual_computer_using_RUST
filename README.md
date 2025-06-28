# Logical Foundation: Building a Virtual Computer in Rust

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

This project implements a 16-bit virtual computer from first principles, following the "Nand to Tetris" methodology with Rust's safety guarantees. It serves as both an educational tool for computer architecture and a practical Rust learning experience.




## 🧩 Key Components

### 1. Fundamental Gates (`circuits/gates.rs`)
Implements all basic logic gates from NAND:
```rust
nand(a, b) -> bool
not(a) -> bool
and(a, b) -> bool
or(a, b) -> bool
xor(a, b) -> bool 
```


## Getting Started

Follow these instructions to get a copy of the project up and running on your local machine for development and testing purposes.

### Prerequisites

  * Rust 1.70+
      * Install via [rustup](https://rustup.rs/) for the easiest setup.

### Installation

  * Clone the repository:

<!-- end list -->

```bash
git clone https://github.com/yourusername/Virtual_computer_using_RUST.git
```

  * Navigate to the project directory:

<!-- end list -->

```bash
cd Virtual_computer_using_RUST
```

  * Build the project:

<!-- end list -->

```bash
cargo build
```

## Running the Project

Execute the main binary:

```bash
cargo run
```

## Testing

Run all tests:

```bash
cargo test
```

## Example Usage

```rust
// In  src/main.rs
use Virtual_computer_using_RUST::{BoolArray, circuits::{arithmetic::{not16,and16}, muxes::*}};  // Virtual_computer_using_RUST is your root directory

fn main() {
    let a = [true; 16];
    let b = [false; 16];

    println!("NOT: {}", BoolArray(not16(a)));
    println!("AND: {}", BoolArray(and16(a, b)));
    println!("MUX: {}", BoolArray(mux16(a, b, true)));
}
```

## Development Principles

### Hardware Accuracy

  * All components built from NAND gates
  * No shortcuts using Rust's native operators


## Roadmap

  * Implement full ALU
  * Add CPU components (registers, PC)
  * Build assembly language compiler
  * Add visual simulation

## Contributing

For now I am building the fundamental block and I am in learning phase I don't think contribution make sense now.But your suggestion will always matter.Feel free to ask questions and give suggestion.

## License

This project is licensed under the MIT License - see the LICENSE file for details.