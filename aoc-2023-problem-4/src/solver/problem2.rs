
use std::{
    result::Result,
    error::Error
};

pub struct Problem2 {
    lines: Vec<String>
}

impl Problem2 {
    pub fn new(lines: Vec<String>) -> Result<Problem2, Box<dyn Error>> {
        Ok(Problem2 {
            lines: lines,
        })
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {
        
        return Ok(0)
    }
}

