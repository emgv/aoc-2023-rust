
use core::num;
use std::{
    result::Result,
    error::Error,
};

use super::shared:: {
    self, Matrix
};

pub struct Problem1 {
    lines: Vec<String>,
    input_matrix: Matrix<char>
}

impl Problem1 {
    pub fn new(lines: Vec<String>) -> Result<Problem1, Box<dyn Error>> {
        Ok(Problem1 {
            input_matrix: Matrix::<char>::to_char_matrix(&lines),
            lines: lines,
        })
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {
        
        if self.lines.is_empty() {
            return Ok(0);
        }

        let mut total: u64 = 0;
        let mut r = 0;
        for row in self.input_matrix.iter() {
            let mut c = 0;
            for cell_value in row {

                if shared::is_symbol(cell_value) {
                    let numbers = shared::get_adjacent_numbers(&self.input_matrix, r, c);
                    total += numbers
                        .iter()
                        .map(|n| u64::from(*n))
                        .sum::<u64>();
                }

                c += 1;
            }
            r += 1;
        }

        Ok(total)
    }
}


