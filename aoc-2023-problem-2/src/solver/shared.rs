use regex::Regex;
use std::{
    error::Error
};

#[derive(Debug)]
pub struct GameRound {
    pub game_id: u32,
    pub results: Option<Vec<GameHandResult>>
}

#[derive(Debug)]
pub enum GameHandResult {
    CubesCount {
        red:u32,
        green:u32,
        blue:u32,
    }
}

pub struct GameParser {
    pattern_gameid: Regex,
    pattern_red_count: Regex,
    pattern_green_count: Regex,
    pattern_blue_count: Regex,
}

impl GameParser {
    pub fn new() -> Result<GameParser, Box<dyn Error>>{
        Ok(GameParser {
            pattern_gameid: Regex::new("[Gg]ame (?P<gameid>[0-9]+):")?,
            pattern_red_count: Regex::new("[^0-9]*(?P<red>[0-9]+) [Rr]ed")?,
            pattern_green_count: Regex::new("[^0-9]*(?P<green>[0-9]+) [Gg]reen")?,
            pattern_blue_count: Regex::new("[^0-9]*(?P<blue>[0-9]+) [Bb]lue")?,
        })
    }

    pub fn convert_to_game_round(&self, input_line: &str) -> Result<GameRound, Box<dyn Error>> {
        let game_id = self
            .parse_game_id(input_line)?
            .ok_or(format!("Could not parse the game id for the input line {:?}", input_line))?;
        let hands_info: Vec<&str> = input_line
            .split(';')
            .collect();

        if hands_info.len() == 0 {
            return Ok(GameRound::new(game_id, None));
        }
        
        // A game round may have multiple results, each result is separated by ";", ie:
        // Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
        //                      ^ separator             ^ separator
        let mut results: Vec<GameHandResult> = vec![];
        for hand in hands_info {
            let red_count = self
                .parse_gamehand_cubes_count(hand, &self.pattern_red_count)?
                .or(Some(0)).unwrap();
            
            let blue_count = self
                .parse_gamehand_cubes_count(hand, &self.pattern_blue_count)?
                .or(Some(0)).unwrap();

            let green_count = self
                .parse_gamehand_cubes_count(hand, &self.pattern_green_count)?
                .or(Some(0)).unwrap();
            
            results.push(GameHandResult::CubesCount { red: red_count, green: green_count, blue: blue_count });
        }

        Ok(GameRound::new(game_id, Some(results)))
    }

    fn parse_game_id(&self, input_line: &str) -> Result<Option<u32>, Box<dyn Error>> {

        let caps = self.pattern_gameid
            .captures(input_line);

        if let None = caps {
            return Ok(None);
        }
        
        let (_, [gameid]) = caps.unwrap().extract();
        Ok(Some(gameid.parse::<u32>()?))
    }

    fn parse_gamehand_cubes_count(&self, hand_info: &str, pattern: &Regex) -> Result<Option<u32>, Box<dyn Error>> {

        let caps = pattern
            .captures(hand_info);

        if let None = caps {
            return Ok(None);
        }
        
        let (_, [cubes_count]) = caps.unwrap().extract();
        Ok(Some(cubes_count.parse::<u32>()?))
    }
}

impl GameRound {
    pub fn new(game_id: u32, hand: Option<Vec<GameHandResult>>) -> GameRound {
        GameRound {
            game_id: game_id,
            results: hand
        }
    }

    pub fn get_max_values_for_each_color(&self) -> Option<GameHandResult> {
        if let None = self.results {
            return None;
        }

        let results = self.results.as_ref().unwrap();
        if results.len() == 0 {
            return None;
        }
        
        let mut min = (0, 0, 0);
        for result in results {
            if min.0 < result.red() {
                min.0 = result.red();
            }

            if min.1 < result.green() {
                min.1 = result.green();
            }

            if min.2 < result.blue() {
                min.2 = result.blue();
            }
        }

        Some(GameHandResult::CubesCount { red: min.0, green: min.1, blue: min.2 })
    }
}

impl GameHandResult {
    pub fn is_less_than_or_equal(&self, other: &GameHandResult) -> bool {
        // TODO: convert this to an implementation of a known comparison trait
        let tuple1 = self.to_tuple();
        let tuple2 = other.to_tuple();

        return tuple1.0 <= tuple2.0
            && tuple1.1 <= tuple2.1
            && tuple1.2 <= tuple2.2;
    }

    fn to_tuple(&self) -> (u32, u32, u32) {
        match self {
            GameHandResult::CubesCount { red, green, blue } => return (*red, *green, *blue)
        }
    }

    pub fn red(&self) -> u32 {
        match self {
            GameHandResult::CubesCount { red, green, blue } => return *red
        }
    }

    pub fn green(&self) -> u32 {
        match self {
            GameHandResult::CubesCount { red, green, blue } => return *green
        }
    }

    pub fn blue(&self) -> u32 {
        match self {
            GameHandResult::CubesCount { red, green, blue } => return *blue
        }
    }
}