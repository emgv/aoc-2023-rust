use std::process;
use std::error::Error;

mod util;
mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() < 3 {
        println!("Please provide two aoc input files, one for problem 1.1, and another for problem 1.2, please respect the order.");
        process::exit(1);
    }

    let file_name_problem1 = &args[1];
    let file_name_problem2 = &args[2];

    let solver = solver::Solver::new(
        file_name_problem1,
        file_name_problem2);
    let problem1_answer = solver.solve_problem1()?;
    println!("Problem1 answer: {:?}", problem1_answer);
    
    let problem1_answer = solver.solve_problem2()?;
    println!("Problem2 answer: {:?}", problem1_answer);

    Ok(())
}
