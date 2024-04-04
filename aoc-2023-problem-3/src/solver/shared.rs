
use core::slice::Iter;

pub struct Matrix<T> {
    data: Vec<Vec<T>>
}

impl<T> Matrix<T> {
    pub fn empty() -> Matrix<T> {
        Matrix {
            data: vec![]
        }
    }

    pub fn new(data: Vec<Vec<T>>) -> Matrix<T> {
        Matrix {
            data: vec![]
        }
    }

    pub fn get(&self, r:usize, c:usize) -> Option<&T> {
        let row = self.data.get(r);
        if let None = row {
            return None;
        }

        row.unwrap().get(c)
    }

    pub fn get_row(&self, r:usize) -> Option<&Vec<T>> {
        self.data.get(r)
    }

    pub fn iter(&self) -> Iter<'_, Vec<T>> {
        self.data.iter()
    }

    pub fn to_char_matrix(lines: &Vec<String>) -> Matrix<char> {

        if lines.len() == 0 {
            return Matrix::<char>::empty();
        }
        
        let matrix_data = lines
            .iter()
            .map(|line| {
                line.chars().collect::<Vec<char>>()
            })
            .collect::<Vec<Vec<char>>>();

        Matrix {
            data: matrix_data
        }
    }
}

pub fn is_symbol(ch: &char) -> bool {

    if ch.is_digit(10) {
        return false;
    }

    match ch {
        '.' => false,
        _ => true
    }
}

pub fn is_gear_symbol(ch: &char) -> bool {

    match ch {
        '*' => true,
        _ => false
    }
}

pub fn get_adjacent_numbers(input_matrix: &Matrix<char>, row_symbol: usize, col_symbol: usize) -> Vec<u32> {
    let east = get_digits_x(input_matrix, row_symbol, col_symbol, 1)
        .unwrap_or(String::from(""));
    let west = get_digits_x(input_matrix, row_symbol, col_symbol, -1)
        .unwrap_or(String::from(""));
    let mut north = get_digits_y(input_matrix, row_symbol, col_symbol, -1)
        .unwrap_or(vec![]);
    let mut south = get_digits_y(input_matrix, row_symbol, col_symbol, 1)
        .unwrap_or(vec![]);

    let mut values = vec![east, west];
    values.append(&mut north);
    values.append(&mut south);

    let mut result = vec![];
    for value in values {
        if value.len() == 0 {
            continue;
        }

        let int_value = value.parse::<u32>();
        if let Err(e) = int_value {
            continue;
        }

        result.push(int_value.unwrap());
    }

    result
}

pub fn get_digits_y(input_matrix: &Matrix<char>, r: usize, c: usize, y_direction: i32) -> Option<Vec<String>> {

    let y = r as i32 + y_direction;
    if y < 0 {
        return None;
    }

    let row_index = y as usize;
    let middle = input_matrix
        .get(row_index, c);

    if let None = middle {
        return None;
    }

    let left = get_digits_x(input_matrix, row_index, c, -1)
        .unwrap_or(String::from(""));
    let right = get_digits_x(input_matrix, row_index, c, 1)
        .unwrap_or(String::from(""));
    let middle_ch = middle.unwrap();
    
    if middle_ch.is_digit(10) {
        return Some(vec![format!("{}{}{}", left, middle_ch, right)]);
    }

    let mut result = vec![];
    if !left.is_empty() {
        result.push(left);
    }

    if !right.is_empty() {
        result.push(right);
    }

    if result.is_empty() {
        return None;
    }
    
    return Some(result);
}

pub fn get_digits_x(input_matrix: &Matrix<char>, r: usize, c: usize, x_direction: i32) -> Option<String> {
    let row = input_matrix.get_row(r);
    if let None = row {
        return None;
    }

    let row_vec = row.unwrap();
    let mut i = c as i32 + x_direction;
    let mut num = String::from("");
    loop {

        if i < 0 {
            break;
        }

        let ch = row_vec.get(i as usize);
        if let None = ch {
            break;
        }

        let ch_value = ch.unwrap();
        if !ch_value.is_digit(10) {
            break;
        }

        if x_direction < 0 {
            num.insert(0, *ch_value);
        }
        else {
            num.push(*ch_value);
        }
        
        i += x_direction;
    }

    if num.is_empty() {
        return None;
    }

    return Some(num);
}
