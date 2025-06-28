# Logical Foundation: Building a Virtual Computer in Rust

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

This project implements a 16-bit virtual computer from first principles, following the "Nand to Tetris" methodology with Rust's safety guarantees. It serves as both an educational tool for computer architecture and a practical Rust learning experience.

## 🏗️ Project Structure
logical_foundation/
├── Cargo.toml          # Project configuration
├── src/
│   ├── lib.rs          # Core library (BoolArray definition)
│   ├── main.rs         # CLI interface and tests
│   └── circuits/       # Hardware components
│       ├── mod.rs      # Module declarations
│       ├── gates.rs    # Fundamental logic gates
│       ├── arithmetic.rs # ALU components
│       └── muxes.rs    # Multiplexers



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
git clone https://github.com/yourusername/logical_foundation.git
```

  * Navigate to the project directory:

<!-- end list -->

```bash
cd logical_foundation
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
use logical_foundation::{BoolArray, circuits::{arithmetic::{not16,and16}, muxes::*}};

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

### Memory Efficiency

  * Stack allocation where possible
  * Explicit heap usage only for dynamic components

### Type Safety

  * Clear separation between computation and display
  * Zero-cost abstractions for hardware simulation

## Roadmap

  * Implement full ALU
  * Add CPU components (registers, PC)
  * Build assembly language compiler
  * Add visual simulation

## Contributing

For now I am building the fundamental block and I am in learning phase I don't think contribution make sense now.But you suggestion will always matter.Feel free to ask questions and give suggestion.

## License

This project is licensed under the MIT License - see the LICENSE file for details.