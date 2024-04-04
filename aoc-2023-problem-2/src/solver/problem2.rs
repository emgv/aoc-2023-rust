
use std::{
    result::Result,
    error::Error
};

pub use super::shared::{
    GameRound,
    GameHandResult,
    GameParser
};

pub struct Problem2 {
    lines: Vec<String>,
    game_parser: GameParser
}

impl Problem2 {
    pub fn new(lines: Vec<String>) -> Result<Problem2, Box<dyn Error>> {
        Ok(Problem2 {
            lines: lines,
            game_parser: GameParser::new()?
        })
    }

    pub fn solve(&self) -> Result<u64, Box<dyn Error>> {

        let good_game_ids: Result<Vec<u64>, Box<dyn Error>> = self.lines
            .iter()
            .map(|line| {
                let game_round = self.game_parser.convert_to_game_round(line)?;
                if let None = game_round.results {
                    return Ok(0);
                }

                let min = game_round.get_max_values_for_each_color();
                if let None = min {
                    return Ok(0);
                }

                let min_value = min.unwrap();

                return Ok(u64::from(min_value.red())
                    * u64::from(min_value.green())
                    * u64::from(min_value.blue()));
            })
            .collect();
        
        Ok(good_game_ids?.iter().sum())
    }
}

