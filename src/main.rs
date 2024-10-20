#![feature(test)]

extern crate test;

// only available in the nightly version
use test::Bencher;
use core::panic;
use std::{collections::{HashSet}, thread, time::Duration};
use colored::Colorize;
use std::io::{stdout, Write};

struct Sudoku {
    init_grid: [u8; 81],
    grid: [u8; 81],
    empty_cell_token: u8,
}

struct PathElement {
    idx_cell: usize,
    valid_digits: Vec<u8>,
    idx_digit: Option<usize>,
}

impl PathElement {
    fn get_digit(&self) -> u8 {
        self.valid_digits[self.idx_digit.unwrap()]
    }

    fn increase_digit(&mut self) -> Result<(), ()>{
        // check if the increase will produce an index error later
        if let Some(idx_digit) = self.idx_digit {
            if idx_digit + 1 >= self.valid_digits.len() {
                Err(())
            } else {
                // increase by 1
                self.idx_digit = Some(self.idx_digit.unwrap() + 1);
                Ok(())
            }
        } else {
            // set to 0 if it was None
            self.idx_digit = Some(0);
            Ok(())
        }
        
    }
}

static hard_grid: [u8; 81] = [
    0,5,0,3,0,2,0,8,0,
    0,0,0,0,8,0,0,0,0,
    0,2,0,1,0,9,0,7,0,
    6,0,0,0,0,0,0,0,5,
    0,0,4,2,0,3,7,0,0,
    9,8,0,0,0,0,0,1,3,
    0,4,0,0,0,0,0,2,0,
    0,0,1,9,0,4,6,0,0,
    0,0,5,0,0,0,1,0,0
];

static easy_grid: [u8; 81] = [
    0,7,0,5,8,3,0,2,0,
    0,5,9,2,0,0,3,0,0,
    3,4,0,0,0,6,5,0,7,
    7,9,5,0,0,0,6,3,2,
    0,0,3,6,9,7,1,0,0,
    6,8,0,0,0,2,7,0,0,
    9,1,4,8,3,5,0,7,6,
    0,3,0,7,0,1,4,9,5,
    5,6,7,4,2,9,0,1,3
];
static solved_grid: [u8; 81] = [
    1,7,6,5,8,3,9,2,4, 
    8,5,9,2,7,4,3,6,1, 
    3,4,2,9,1,6,5,8,7, 
    7,9,5,1,4,8,6,3,2, 
    4,2,3,6,9,7,1,5,8, 
    6,8,1,3,5,2,7,4,9, 
    9,1,4,8,3,5,2,7,6, 
    2,3,8,7,6,1,4,9,5, 
    5,6,7,4,2,9,8,1,3,
];

impl Sudoku{
    fn new(grid: [u8; 81]) -> Sudoku {
        // create a new object with the input grid
        let sudoku = Sudoku {
            init_grid: grid,
            grid,
            empty_cell_token: 0,
        };
        sudoku
    }

    fn get_row(&self, i: usize) -> &[u8] {
        // return the i-th row
        &self.grid[i*9..(i+1)*9]
    }

    fn get_column(&self, i: usize) -> [u8; 9] {
        // return the i-th column
        let mut col: [u8; 9] = [0; 9];
        for k in 0..9 {
            col[k] = self.grid[i + k * 9]
        }
        col
    }

    fn get_square(&self, i: usize, j: usize) -> [u8; 9] {
        // return the square of the cell at row i and column j
        let mut square: [u8; 9] = [0; 9];
        for k in 0..3 {
            for l in 0..3 {
                square[k*3 + l] = self.grid[(i/3*3 + k)* 9 + (j/3*3 + l)] 
            }
        }
        square
    }

    fn show(&self) {
        let mut lock = stdout().lock();
        write!(lock, "\n").unwrap();
        for i in 0..81 {
            let cell = self.grid[i];
            if cell == self.empty_cell_token {
                write!(lock, "{} ", ".".blue()).unwrap();
            } else if self.init_grid[i] == self.empty_cell_token {
                write!(lock, "{} ", cell.to_string().green()).unwrap();
            } else {
                write!(lock, "{} ", cell).unwrap();
            }
            if i % 3 == 2 {
                write!(lock, "| ").unwrap();
            }
            if i % 27 == 26 {
                write!(lock, "\n{}", &"-".repeat(23)).unwrap();
            }
            if i % 9 == 8 {
                write!(lock, "\n").unwrap();
            }
        }
        stdout().flush().unwrap();
    }

    fn get_empty_cells(&self) -> Vec<usize> {
        // get indexes of empty cells in the grid
        let mut indexes: Vec<usize> = Vec::new();
        for i in 0..9 {
            for j in 0..9 {
                let idx = i * 9 + j;
                if self.grid[idx] == self.empty_cell_token {
                    indexes.push(idx);
                }
            }
        }
        indexes
    }

    fn get_valid_digits(&self, i: usize, j: usize) -> Result<Vec<u8>, ()> {
        // given cell with row i and column j, return possible digits
        let mut possible_digits: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
        for d in &self.get_column(j) {
            if let Some(idx) = possible_digits.iter().position(|c| c==d) {
                possible_digits.remove(idx);
            }
        }
        for d in self.get_row(i) {
            if let Some(idx) = possible_digits.iter().position(|c| c==d) {
                possible_digits.remove(idx);
            }
        }
        for d in &self.get_square(i, j) {
            if let Some(idx) = possible_digits.iter().position(|c| c==d) {
                possible_digits.remove(idx);
            }
        }
        if possible_digits.is_empty() {
            Err(())
        } else {
            Ok(possible_digits)
        }
    }

    fn fill_one_possibility_cells(&mut self) -> u8 {
        // find all cells with only one possibility and set the value
        // for some grid, it may reduce the search space greatly
        let mut updated_cells: u8 = 0;
        for i in 0..9 {
            for j in 0..9 {
                if self.grid[i*9+j] == self.empty_cell_token {
                    let digits = self.get_valid_digits(i, j).unwrap();
                    if digits.len() == 1 {
                        self.grid[i*9+j] = digits[0];
                        updated_cells += 1;
                    }
                }
            }
        }
        updated_cells
    }

    fn validate(&self) -> bool {
        // check if the sudoku grid is finished and correct
        let digits: HashSet<u8> = HashSet::from([1,2,3,4,5,6,7,8,9]);
        for i in 0..9 {
            let row = self.get_row(i);
            let set = HashSet::from_iter(row.iter().cloned());
            if digits != set{
                println!("Row {} is wrong", i);
                return false;
            }
        }
        for i in 0..9 {
            let column = self.get_column(i);
            let set = HashSet::from_iter(column.iter().cloned());
            if digits != set {
                println!("Column {} is wrong", i);
                return false;
            }
        }
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let square = self.get_square(i, j);
                let set = HashSet::from(square);
                if digits != set {
                    println!("Square in i={} and j={} is wrong", i, j);
                    return false;
                }
            }
        }
        true
    }

    fn brute_force(&mut self, show: bool) {
        // To reduce the search space, fill cells with only one possibility
        let n_filled_cells = self.fill_one_possibility_cells();
        println!("{} cells filled with only one possibility", n_filled_cells);

        let empty_cells = self.get_empty_cells();
        println!("Empty cells are {:?}", empty_cells);

        // Initialize variables
        let mut path: Vec<PathElement> = Vec::new();
        let mut increment_path: bool = true;

        while path.len() < empty_cells.len() {
            if increment_path {
                    let idx_cell = empty_cells[path.len()];
                    let id_row = idx_cell/9;
                    let id_col = idx_cell%9;
                    match self.get_valid_digits(id_row, id_col) {
                        Ok(valid_digits) => {
                            path.push(PathElement {
                                idx_cell, 
                                valid_digits,
                                idx_digit: None
                            });
                        },
                        Err(_) => {}
                    }
                increment_path = false;
            }
                    let last_elt: &mut PathElement = path.last_mut().unwrap();
                    match last_elt.increase_digit() {
                        // idx_digits increased by 1 is valid
                Ok(_) => {
                            self.grid[last_elt.idx_cell] = last_elt.get_digit();
                    increment_path = true;
                        },
                        // idx_digit is superior to valid_digits
                Err(_) => {
                            self.grid[last_elt.idx_cell] = self.empty_cell_token;
                            path.pop().unwrap();
                        }
                    };
                    if show {
                    self.show();
                    // sleep between each grid
                    let dur = Duration::from_millis(10);
                    thread::sleep(dur);
            }
        }
    
        // Check if the grid is finished
        if self.validate() {
            println!("Finished successfully!");
            self.show();
        } else {
            panic!("Something went wrong!")
        }
    }
}

fn main() {
    let mut sudoku = Sudoku::new(hard_grid);
    sudoku.show();

    // Run brute force approach
    sudoku.brute_force(true);
}

#[bench]
fn bench_brute_force(bencher: &mut Bencher) {
    bencher.iter(|| {
        let mut sudoku = Sudoku::new(hard_grid);
        sudoku.brute_force(false);
    })
}