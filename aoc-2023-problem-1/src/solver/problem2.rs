
use crate::util::strings;
use std::{
    result::Result,
    error::Error,
    collections::HashMap
};

pub struct Problem2 {
    lines: Vec<String>,
    numbers: HashMap<String, u32>
}

impl Problem2 {
    pub fn new(lines: Vec<String>) -> Problem2 {
        Problem2 {
            lines: lines,
            numbers: ["one","two","three",
                      "four","five","six",
                      "seven","eight","nine"]
                        .iter()
                        .enumerate()
                        .map(|(i,s)| (s.to_string(), u32::try_from(i+1).unwrap()))
                        .collect()
        }
    }

    fn find_word_number(&self, line: &str) -> (Option<usize>, u32) {
        let mut number_found: (Option<usize>, u32) = (None, 0);

        for number in self.numbers.iter() {
            let found = line.find(number.0);

            if let Some(pos) = found {
                if let None = number_found.0 {
                    number_found = (Some(pos), number.1.to_owned());
                    continue;
                }

                if number_found.0.unwrap() > pos {
                    number_found = (Some(pos), number.1.to_owned());
                }
            }
        }

        if let None = number_found.0 {
            return (None, 0);
        }

        return number_found
    }

    fn rfind_word_number(&self, line: &str) -> (Option<usize>, u32) {
        let mut number_found: (Option<usize>, u32) = (None, 0);

        for number in self.numbers.iter() {
            let found = line.rfind(number.0);
            if let Some(pos) = found {
                if let None = number_found.0 {
                    number_found = (Some(pos), number.1.to_owned());
                    continue;
                }

                if number_found.0.unwrap() < pos {
                    number_found = (Some(pos), number.1.to_owned());
                }
            }
        }

        if let None = number_found.0 {
            return (None, 0);
        }

        return number_found;
    }

    fn find_number(&self, line: &str) -> Result<(Option<usize>, u32), Box<dyn Error>> {
        let index_digit = line
            .find(char::is_numeric)
            .map_or((None, Ok(0)),
                |v| {
                    return (Some(v), strings::get_digit_at(line, v))
                });
        let index_numeric_word = self.find_word_number(line);

        index_digit.0
            .map_or(Ok((index_numeric_word.0, index_numeric_word.1)),
                |v| {
                    if let None = index_numeric_word.0 {
                        return Ok((index_digit.0, index_digit.1?));
                    }
                    
                    if index_numeric_word.0.unwrap() < v {
                        return Ok((index_numeric_word.0, index_numeric_word.1));
                    }

                    return Ok((index_digit.0, index_digit.1?));
            })
    }

    fn rfind_number(&self, line: &str) -> Result<(Option<usize>, u32), Box<dyn Error>> {
        let index_digit = line
            .rfind(char::is_numeric)
            .map_or((None, Ok(0)),
                |v| {
                    return (Some(v), strings::get_digit_at(line, v))
                });
        let index_numeric_word = self.rfind_word_number(line);
        
        index_digit.0
            .map_or(Ok((index_numeric_word.0, index_numeric_word.1)),
                |v| {
                    if let None = index_numeric_word.0 {
                        return Ok((index_digit.0, index_digit.1?));
                    }
                    
                    if index_numeric_word.0.unwrap() > v {
                        return Ok((index_numeric_word.0, index_numeric_word.1));
                    }

                    return Ok((index_digit.0, index_digit.1?));
            })
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {

        let numbers: Vec<Result<u32, Box<dyn Error>>> = self.lines
            .iter()
            .map(|line| self.get_num(line))
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

    fn get_num(&self, line: &str) -> Result<u32, Box<dyn Error>> {
        
        let index = self.find_number(line)?;
        if let None = index.0 {
            return Ok(0)
        }

        let index1 = index.0.unwrap();
        let index_rev = self.rfind_number(line)?;
        let index2 = index_rev.0.unwrap();

        if index1 == index2 {
            let value = index.1;
            return Ok(value * 10 + value);
        }
        
        return Ok(index.1 * 10 + index_rev.1);
    }
}

