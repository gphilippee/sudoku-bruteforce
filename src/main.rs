use core::panic;
use colored::Colorize;
use std::io::{stdout, Write};

struct Sudoku {
    init_grid: [u8; 81],
    grid: [u8; 81],
    empty_cell_token: u8,
}

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

    fn has_empty_cells(&self) -> bool {
        // go through the grid in a row-major order
        // returns directly if an empty cell is found
        for i in 0..81 {
            if self.grid[i] == self.empty_cell_token {
                return true
            }
        }
        false
    }

    fn empty_cells(&self) -> u8 {
        // count number of empty cells
        let mut counter: u8 = 0;
        for elt in &self.grid {
            if elt == &self.empty_cell_token {
                counter += 1;
            }
        }
        counter
    }

    fn next_empty_cell(&self) -> Option<usize> {
        // return index of an empty cell in the grid (row-major order)
        // if no cell is empty, return None
        for i in 0..9 {
            for j in 0..9 {
                if self.grid[i*9+j] == self.empty_cell_token {
                    return Some(i*9 + j);
                }
            }
        }
        None
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

    fn valid_digits(&self, i: usize, j: usize) -> Vec<u8> {
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
        possible_digits
    }

    fn fill_one_possibility_cells(&mut self) -> u8 {
        // find all cells with only one possibility and set the value
        // for some gri,d it may reduce the search space greatly
        let mut updated_cells: u8 = 0;
        for i in 0..9 {
            for j in 0..9 {
                if self.grid[i*9+j] == self.empty_cell_token {
                    let digits = self.valid_digits(i, j);
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

 
    fn brute_force(&mut self) {
        // To reduce the search space, fill cells with only one possibility
        let filled_cells = self.fill_one_possibility_cells();
        println!("{} cells filled with only one possibility", filled_cells);

        // When we add a number in the sudoku grid, we will add:
        // (row number * 9 + column number, index in valid digits) in the path
        // It serve us as a memory of recent updates
        let mut path: Vec<(usize, usize)> = Vec::new();
        let mut digits_index_to_try: usize = 0;
        while let Some(grid_idx) = self.next_empty_cell() {
            let id_row = grid_idx/9;
            let id_col = grid_idx%9;
            let digits = self.valid_digits(id_row, id_col);
            if let Some(d) = digits.get(digits_index_to_try) {
                self.grid[grid_idx] = *d;
                path.push((grid_idx, digits_index_to_try));
                digits_index_to_try = 0;
            } else if self.has_empty_cells() {
                // No digits available for this position in `curr_grid`
                // Check if the sudoku is solved

                // The current branch is false, we have to retry higher
                // Pop the last element
                let (idx, idx_digits) = path.pop().unwrap();
                digits_index_to_try = idx_digits + 1;
                self.grid[idx] = self.empty_cell_token;
            }
            self.show();
            let dur = Duration::from_millis(10);
            thread::sleep(dur);
        }
        // No more zeroes
        // Check if the grid is finished
        if self.validate() {
            println!("Finished successfully");
            self.show();
        } else {
            panic!("Something went wrong!")
        }
    }
}

fn main() {
    let easy_grid: [u8; 81] = [
        0,7,0,5,8,3,0,2,0,
        0,5,9,2,0,0,3,0,0,
        3,4,0,0,0,6,5,0,7,
        7,9,5,0,0,0,6,3,2,
        0,0,3,6,9,7,1,0,0,
        6,8,0,0,0,2,7,0,0,
        9,1,4,8,3,5,0,7,6,
        0,3,0,7,0,1,4,9,5,
        5,6,7,4,2,9,0,1,3];
    let hard_grid: [u8; 81] = [
        0,5,0,3,0,2,0,8,0,
        0,0,0,0,8,0,0,0,0,
        0,2,0,1,0,9,0,7,0,
        6,0,0,0,0,0,0,0,5,
        0,0,4,2,0,3,7,0,0,
        9,8,0,0,0,0,0,1,3,
        0,4,0,0,0,0,0,2,0,
        0,0,1,9,0,4,6,0,0,
        0,0,5,0,0,0,1,0,0];
    let mut sudoku = Sudoku::new(hard_grid);
    sudoku.show();

    // Run brute force approach
    sudoku.brute_force();
}
