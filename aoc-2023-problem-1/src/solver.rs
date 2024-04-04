
mod problem1;
mod problem2;
use std:: {
    fs::File,
    io::{
        BufRead,
        BufReader
    },
    path::{Path, PathBuf},
    error::Error,
    result::Result
};

pub struct Solver {
    input_file_problem1: PathBuf,
    input_file_problem2: PathBuf
}

impl Solver {
    pub fn new(input_file_problem1: &impl AsRef<Path>, input_file_problem2: &impl AsRef<Path>) -> Self {
        
        Self {
            input_file_problem1: input_file_problem1.as_ref().to_owned(),
            input_file_problem2: input_file_problem2.as_ref().to_owned(),
        }
    }

    fn read_file(&self, input_file: &PathBuf) -> Result<Vec<String>, Box<dyn Error>> {
        let file = File::open(input_file.as_path())?;
        let mut reader = BufReader::new(file);
        
        let mut lines: Vec<String> = vec![];
        loop {

            let mut line_read =  String::from("");
            let bytes_read = reader.read_line(&mut line_read)?;
            
            if bytes_read == 0 {
                break;
            }

            line_read = line_read
                .strip_suffix("\r\n")
                .or(line_read.strip_suffix("\n"))
                .unwrap_or(line_read.as_str())
                .to_string();

            lines.push(line_read);
        }

        Ok(lines)
    }

    pub fn solve_problem1(&self) -> Result<u64, Box<dyn Error>> {
        let lines = self.read_file(&self.input_file_problem1)?;

        let problem1_solver = problem1::Problem1::new(lines);
        problem1_solver.solve()
    }

    pub fn solve_problem2(&self) -> Result<u64, Box<dyn Error>> {
        let lines = self.read_file(&self.input_file_problem2)?;

        let problem2_solver = problem2::Problem2::new(lines);
        problem2_solver.solve()
    }
}

