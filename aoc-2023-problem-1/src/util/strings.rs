use std::{
    result::Result,
    error::Error
};

pub fn get_digit_at(string_value: &str, index: usize) -> Result<u32, Box<dyn Error>> {
    let num = string_value
        .chars()
        .nth(index)
        .ok_or(format!("Could not get the char at index {:?}", index).to_owned())?
        .to_digit(10)
        .ok_or(format!("Could not convert the char to digit at index {:?}", index).to_owned())?;

    return Ok(num)
}
