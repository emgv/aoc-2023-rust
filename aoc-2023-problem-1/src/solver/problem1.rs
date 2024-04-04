
use crate::util::strings;
use std::{
    result::Result,
    error::Error
};

pub struct Problem1 {

    lines: Vec<String>
}

// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/first-edition/error-handling.html
// ^ Always avoid unwrapping, instead return Result<T, E> -- let the caller decide what to do with the error
//   becasue unwrapp throws if there is an error
impl Problem1 {

    pub fn new(lines: Vec<String>) -> Problem1 {
        Problem1 {
            lines: lines
        }
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {

        let numbers: Vec<Result<u32, Box<dyn Error>>> = self.lines
            .iter()
            .map(|line| Self::get_num(line))
            .collect();
        
        let mut result: u64 = 0;
        for number in numbers {
            if let Err(err) = number {
                return Err(err)
            }

            result  += u64::from(number.unwrap());
        }

        return Ok(result);
    }

    fn get_num(line: &str) -> Result<u32, Box<dyn Error>> {
        
        let index = line.find(char::is_numeric);
        if let None = index {
            return Ok(0)
        }

        let index1 = index.unwrap();
        let index2 = line
            .rfind(char::is_numeric)
            .unwrap();

        let num1 = strings::get_digit_at(line, index1)?;
        if index1 == index2 {
            return Ok(num1 * 10 + num1);
        }
        
        let num2 = strings::get_digit_at(line, index2)?;
        return Ok(num1 * 10 + num2);
    }
}
