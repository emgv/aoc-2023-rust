
use std::{
    result::Result,
    error::Error
};

pub struct Problem2 {
    lines: Vec<String>,
    input_matrix: Matrix<char>
}

use super::shared:: {
    self, Matrix
};

impl Problem2 {
    pub fn new(lines: Vec<String>) -> Result<Problem2, Box<dyn Error>> {
        Ok(Problem2 {
            input_matrix: Matrix::<char>::to_char_matrix(&lines),
            lines: lines,
        })
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {
        
        if self.lines.len() == 0 {
            return Ok(0);
        }

        let mut total: u64 = 0;
        let mut r = 0;
        for row in self.input_matrix.iter() {
            let mut c = 0;
            for cell_value in row {

                if shared::is_gear_symbol(cell_value) {
                    let numbers = shared::get_adjacent_numbers(&self.input_matrix, r, c);
                    if numbers.len() == 2 {
                        total += u64::from(numbers[0]) * u64::from(numbers[1]);
                        //println!("{}*{}={}", u64::from(numbers[0]), u64::from(numbers[1]), u64::from(numbers[0]) * u64::from(numbers[1]));
                    }
                }

                c += 1;
            }
            r += 1;
        }

        Ok(total)
    }
}

