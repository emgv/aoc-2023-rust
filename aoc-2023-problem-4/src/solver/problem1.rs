
use std::{
    result::Result,
    error::Error,
};

use super::shared::Hailstone;

pub struct Problem1 {
    lines: Vec<String>
}

impl Problem1 {
    pub fn new(lines: Vec<String>) -> Result<Problem1, Box<dyn Error>> {

        Ok(Problem1 {
            lines: lines,
        })
    }

    pub fn solve(&self, min_value_for_x_and_y: f64, max_value_for_x_and_y: f64) -> Result<u64, Box<dyn Error>> {
        
        if self.lines.is_empty() {
            return Ok(0);
        }

        let hailstones = self.lines
            .iter()
            .map(|line| {
                let hailstone = Hailstone::parse(&line)?;
                return Ok(hailstone)
            })
            .collect::<Result<Vec<Hailstone>, Box<dyn Error>>>()?;
        
        let mut intersections_count: u64 = 0;
        let count_hailstones = hailstones.len();

        for i in 0 .. count_hailstones {

          let hailstone1 = &hailstones[i];

          for j in i+1 .. count_hailstones {

            let hailstone2 = &hailstones[j];
            let intersection_result = Self::intersection_point(hailstone1, hailstone2);

            if let None = intersection_result {
              continue;
            }

            let intersection = intersection_result.unwrap();
            if intersection.0 >= min_value_for_x_and_y && intersection.0 <= max_value_for_x_and_y
              && intersection.1 >= min_value_for_x_and_y && intersection.1 <= max_value_for_x_and_y
              && !Self::is_point_from_the_past(hailstone1, &intersection)
              && !Self::is_point_from_the_past(hailstone2, &intersection) {

                // println!("{:?}  and  {:?}  at  {:?} , {:?}", hailstone1.info, hailstone2.info, intersection.0, intersection.1);
                intersections_count += 1;
              }
          }
        }

        Ok(intersections_count)
    }

    fn intersection_point(h1: &Hailstone, h2: &Hailstone) -> Option<(f64, f64)> {
      let m1 = if h1.vx != 0f64 { h1.vy / h1.vx } else { 0f64 };
      let m2 = if h2.vx != 0f64 { h2.vy / h2.vx } else { 0f64 };
      let b1 = h1.y - m1 * h1.x;
      let b2 = h2.y - m2 * h2.x;

      if m1 == m2 {
        return None
      }

      let intersection_x = (b2 - b1) / (m1 - m2);
      return Some((
        intersection_x,
        m1 * intersection_x + b1
      ))
    }

    fn is_point_from_the_past(hailstone: &Hailstone, point: &(f64, f64)) -> bool {

      let vx = point.0 - hailstone.x;
      let vy = point.1 - hailstone.y;

      return vx.signum() !=  hailstone.vx.signum() || vy.signum() !=  hailstone.vy.signum()
    }
}


