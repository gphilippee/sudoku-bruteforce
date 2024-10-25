# Sudoku Solver in Rust (Brute-Force Approach)
This repository contains a Rust implementation of a brute-force algorithm to solve a 9x9 Sudoku puzzle. The brute-force method works by trying all possible numbers in empty cells until a valid solution is found or the puzzle is determined to be unsolvable.

<img src="https://github.com/user-attachments/assets/0b53a1cf-d28f-4e84-985c-45846138f861" alt="sudoku_bruteforce" width="200"/>

## Features
* Solves standard 9x9 Sudoku puzzles.
* Implements a backtracking algorithm to try possible solutions.
* Provides a simple command-line interface for inputting puzzles.
* Fast and efficient for small Sudoku puzzles.

## How It Works
This implementation uses a backtracking algorithm, which is a brute-force technique to solve the puzzle. It works by:

1. Placing a number (1-9) in an empty cell.
2. Checking if the number is valid based on the Sudoku rules (no repeating numbers in the row, column, or 3x3 grid).
3. Recursively attempting to place numbers in the remaining empty cells.
4. Backtracking if a conflict arises and trying a different number.

The process continues until the puzzle is completely solved or all possibilities are exhausted.

## Getting Started

### Prerequisites
Rust installed on your system.

### Installation
Clone the repository:
```bash
git clone https://github.com/yourusername/sudoku-solver-rust.git
cd sudoku-solver-rust
```

Build the project using Cargo:
```bash
cargo build --release
```

### Running the Solver
You can run the Sudoku solver by providing a Sudoku puzzle in the form of a 2D array where 0 represents empty cells.

```bash
cargo run
```

You can modify the input puzzle in the code or extend the application to accept input from a file or command-line.

## Contributing
Contributions are welcome! If you have any suggestions or improvements, feel free to submit a pull request.

## License
This project is licensed under the MIT License.
