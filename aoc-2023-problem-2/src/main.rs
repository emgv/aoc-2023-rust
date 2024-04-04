use std::{
    error::Error,
    process
};

use regex::Regex;

mod solver;

fn main() -> Result<(), Box<dyn Error>> {
    /*
    let some_test = "Game 1: 32 blue, 4 red; 1 red, 2 green, 19 blue; 2 green";
    let pattern = Regex::new("[Gg]ame (?P<gameid>[0-9]+):[^0-9]*(?P<blue>[0-9]+) [Bb]lue")?;
    let caps = pattern.captures(some_test);
    if let None = caps {
        println!("No caps found");
        return Ok(());
    }

    let (_, [gameid, blue]) = caps.unwrap().extract();
    println!("game-id: {:?}, blue: {:?}", gameid, blue);

    let some_test2 = "Game 1: 31 blue, 4 red; 1 red, 2 green, 23 blue; 2 green";
    let caps2 = pattern.captures(some_test2); // reusing the same pattern
    let (_, [gameid2, blue2]) = caps2.unwrap().extract();
    println!("game-id2: {:?}, blue2: {:?}", gameid2, blue2);

    Ok(())
    */

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