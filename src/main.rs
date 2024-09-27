use core::panic;
use std::{collections::{HashMap, HashSet}, hash::Hash, thread, time::Duration};

struct Sudoku {
    grid: [u8; 81],
    empty_cell_token: u8,
    state: HashMap<[u8; 81], [u8; 81]>
}

impl Sudoku{
    fn new(grid: [u8; 81]) -> Sudoku {
        let sudoku = Sudoku {
            grid,
            empty_cell_token: 0,
            state: HashMap::new()
        };
        sudoku
    }

    fn get_row(&self, i: usize, j: usize) -> &[u8] {
        &self.grid[i*9..(i+1)*9]
    }

    fn get_column(&self, i: usize, j: usize) -> [u8; 9] {
        let mut col: [u8; 9] = [0; 9];
        for k in 0..9 {
            col[k] = self.grid[j + k * 9]
        }
        col
    }

    fn get_square(&self, i: usize, j: usize) -> [u8; 9] {
        let mut square: [u8; 9] = [0; 9];
        for k in 0..3 {
            for l in 0..3 {
                square[k*3 + l] = self.grid[(i/3*3 + k)* 9 + (j/3*3 + l)] 
            }
        }
        square
    }

    fn show(&self) {
        // print!("{}[2J", 27 as char);
        for i in 0..9 {
            let mut row: String = String::new();
            for j in 0..9 {
                row += &format!("{} ", self.grid[i * 9 + j]);
                if j % 3 == 2 {
                    row += "| ";
                }
            }
            println!("{}", row);
            if i % 3 == 2 {
                let dash = "-".repeat(23);
                println!("{}", dash);
            }
        }
    }

    fn valid_digits(&self, i: usize, j: usize) -> Vec<u8> {
        let mut possible_digits: Vec<u8> = vec![1,2,3,4,5,6,7,8,9];
        // println!("{:?}", self.get_column(i, j));
        // println!("{:?}", self.get_row(i, j));
        // println!("{:?}", self.get_square(i, j));
        for d in &self.get_column(i, j) {
            if let Some(idx) = possible_digits.iter().position(|c| c==d) {
                possible_digits.remove(idx);
            }
        }
        for d in self.get_row(i, j) {
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

    fn empty_cells(&self) -> u8 {
        let mut counter: u8 = 0;
        for elt in &self.grid {
            if elt == &self.empty_cell_token {
                counter += 1;
            }
        }
        counter
    }

    fn fill_one_possibility_cells(&mut self) -> u8 {
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
        let digits: HashSet<u8> = HashSet::from([1,2,3,4,5,6,7,8,9]);
        for i in 0..9 {
            let row = self.get_row(i, 0);
            let set = HashSet::from_iter(row.iter().cloned());
            if digits != set{
                // println!("Row {} is wrong", i);
                return false;
            }
        }
        for i in 0..9 {
            let column = self.get_column(0, i);
            let set = HashSet::from_iter(column.iter().cloned());
            if digits != set {
                // println!("Column {} is wrong", i);
                return false;
            }
        }
        for i in (0..9).step_by(3) {
            for j in (0..9).step_by(3) {
                let square = self.get_square(i, j);
                let set = HashSet::from(square);
                if digits != set {
                    // println!("Square in i={} and j={} is wrong", i, j);
                    return false;
                }
            }
        }
        true
    }

    fn next_zero(&self) -> Option<usize> {
        for i in 0..9 {
            for j in 0..9 {
                if self.grid[i*9+j] == self.empty_cell_token {
                    return Some(i*9 + j);
                }
            }
        }
        None
    }
 
    fn brute_force(&mut self) {
        // When we add a number in the sudoku grid, we will add:
        // (row number * 9 + column number, index in valid digits)
        let mut path: Vec<(usize, usize)> = Vec::new();
        // let mut curr_grid = self.grid;
        let mut idx_digits_to_retry : Option<usize> = None;
        while let Some(idx) = self.next_zero() {
            // self.show();
            println!("Path {:?}", path);
            let id_row = idx/9;
            let id_col = idx%9;
            let digits = self.valid_digits(id_row, id_col);
            let idx_digits = match idx_digits_to_retry {
                Some(idx) => idx + 1,
                _ => 0
            };
            // println!("Try index {}", idx_digits);
            println!("Digits: {:?}, index: {}", digits, idx_digits);
            if let Some(d) = digits.get(idx_digits) {
                self.grid[idx] = *d;
                path.push((idx, idx_digits));
                idx_digits_to_retry = None;
            } else {
                // No digits available for this position in `curr_grid`
                // Check if the sudoku is solved
                if self.validate() {
                    println!("Sudoku solved");
                    return
                } else {
                    // The current branch is false, we have to retry higher
                    // Pop the last element
                    let (idx, idx_digits) = path.pop().unwrap();
                    // println!("This path is going nowhere! Index {}", idx);
                    idx_digits_to_retry = Some(idx_digits);
                    // println!("Path to retry: {:?}", path);
                    self.grid[idx] = 0;
                }
            }
            // let dur = Duration::from_secs(1);
            // thread::sleep(dur);
        }
        // No more zeroes
        // Check if the grid is finished
        if self.validate() {
            println!("Finished successfully");
            self.show();
        } else {
            panic!("Something went wrong")
        }
        // for _ in 0..30 {
        //     self.show();
        //     println!("Path {:?}", path);
        //     if let Some(idx) = self.next_zero() {
        //         let id_row = idx/9;
        //         let id_col = idx%9;
        //         let digits = self.valid_digits(id_row, id_col);
        //         let idx_digits = match idx_digits_to_retry {
        //             Some(idx) => idx + 1,
        //             _ => 0
        //         };
        //         // println!("Try index {}", idx_digits);
        //         println!("Digits: {:?}, index: {}", digits, idx_digits);
        //         if let Some(d) = digits.get(idx_digits) {
        //             self.grid[idx] = *d;
        //             path.push((idx, idx_digits));
        //             idx_digits_to_retry = None;
        //         } else {
        //             // No digits available for this position in `curr_grid`
        //             // Check if the sudoku is solved
        //             if self.validate() {
        //                 println!("Sudoku solved");
        //                 return
        //             } else {
        //                 // The current branch is false, we have to retry higher
        //                 // Pop the last element
        //                 let (idx, idx_digits) = path.pop().unwrap();
        //                 // println!("This path is going nowhere! Index {}", idx);
        //                 idx_digits_to_retry = Some(idx_digits);
        //                 // println!("Path to retry: {:?}", path);
        //                 self.grid[idx] = 0;
        //             }
        //         }
        //     } else {
        //         // No more zeroes
        //         // Check if the grid is finished
        //         if self.validate() {
        //             println!("Finished successfully")
        //         } else {
        //             panic!("Something went wrong")
        //         }
        //         break
        //     }
        // }
    }
}

fn main() {
    let range_grid: [u8; 81] = std::array::from_fn(|i| i as u8); 
    let empty_grid: [u8; 81] = [0; 81];
    let finished_grid: [u8; 81] = [
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9,
        1,2,3,4,5,6,7,8,9];
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
    println!("Grid {:?}", sudoku.grid);
    println!("Row {:?}", sudoku.get_row(1,1));
    println!("Column {:?}", sudoku.get_column(1,1));
    println!("Square {:?}", sudoku.get_square(8,8));
    println!("Empty cells {:?}", sudoku.empty_cells());

    // 1. Complete sudoku using cells with only one possibility
    // loop {
    //     let updated_cells = sudoku.fill_one_possibility_cells();
    //     // println!("{}", updated_cells);
    //     if updated_cells == 0 {
    //         break;
    //     }
    //     println!("Empty cells remaining {:?}", sudoku.empty_cells());
    // }
    // sudoku.show();
    // if sudoku.validate() {
    //     println!("Grid is correct")
    // } else {
    //     println!("Grid isn't correct") 
    // };

    // 2. Brute force
    sudoku.brute_force();
}
