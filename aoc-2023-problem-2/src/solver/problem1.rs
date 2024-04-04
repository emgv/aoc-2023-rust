
use core::{
    iter::Enumerate,
    str::Split
};

use std::{
    result::Result,
    error::Error, path::Iter,
};

use regex::Regex;

pub use super::shared::{
    GameRound,
    GameHandResult,
    GameParser
};

pub struct Problem1 {
    lines: Vec<String>,
    game_parser: GameParser
}

impl Problem1 {
    pub fn new(lines: Vec<String>) -> Result<Problem1, Box<dyn Error>> {
        Ok(Problem1 {
            lines: lines,
            game_parser: GameParser::new()?
        })
    }

    pub fn solve(&self, max_game_hand_result: GameHandResult) -> Result<u64, Box<dyn Error>> {

        let good_game_ids: Result<Vec<u64>, Box<dyn Error>> = self.lines
            .iter()
            .map(|line| {
                let game_round = self.game_parser.convert_to_game_round(line)?;
                if let None = game_round.results {
                    return Ok(0);
                }

                let found_big_result = game_round.results
                    .unwrap()
                    .iter()
                    .any(|result| {
                        !result.is_less_than_or_equal(&max_game_hand_result)
                    });
                
                if found_big_result {
                    return Ok(0);
                }

                return Ok(u64::from(game_round.game_id));
            })
            .collect();
        
        Ok(good_game_ids?.iter().sum())
    }
}


