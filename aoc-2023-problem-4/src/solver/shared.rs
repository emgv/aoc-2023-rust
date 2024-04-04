
use std::error::Error;

pub struct Hailstone {
    pub info: String,
    // position
    pub x: f64,
    pub y: f64,
    pub z: f64,
    // velocity
    pub vx: f64,
    pub vy: f64,
    pub vz: f64,
}

impl Hailstone {

    /*
    19, 13, 30 @ -2,  1, -2
    18, 19, 22 @ -1, -1, -2
    */
    pub fn parse(hailstone_info: &str) -> Result<Hailstone, Box<dyn Error>> {
        if hailstone_info.is_empty() {
            return Err("Empty hailstore info given".into())
        }

        let info: Vec<&str> = hailstone_info
            .split('@')
            .collect();
        
        if info.is_empty() || info.len() < 2 {
            return Err("Unable to parse the hail stone info, it should follow the pattern: 19, 13, 30 @ -2,  1, -2".into())
        }
        
        let pos_info = Self::parse_vec3(info[0])?;
        let vel_info = Self::parse_vec3(info[1])?;

        if let None = pos_info {
            return Err("Unable to parse the hail stone info, it should follow the pattern: 19, 13, 30 @ -2,  1, -2".into())
        }

        if let None = vel_info {
            return Err("Unable to parse the hail stone info, it should follow the pattern: 19, 13, 30 @ -2,  1, -2".into())
        }

        let pos = pos_info.unwrap();
        let vel = vel_info.unwrap();

        return Ok(
            Hailstone {
                info: hailstone_info.into(),
                x: f64::from(pos.0),
                y: f64::from(pos.1),
                z: f64::from(pos.2),

                vx: f64::from(vel.0),
                vy: f64::from(vel.1),
                vz: f64::from(vel.2),
            }
        )
    }

    // 19, 13, 30
    fn parse_vec3(vec_info: &str) -> Result<Option<(f64, f64, f64)>, Box<dyn Error>> {
        let vec3: Vec<&str> = vec_info
            .split(',')
            .collect();

        if vec3.is_empty() || vec3.len() < 2 {
            return Ok(None)
        }

        let x = vec3[0].trim().parse::<f64>()?;
        let y = vec3[1].trim().parse::<f64>()?;
        let z = vec3[2].trim().parse::<f64>()?;

        return Ok(Some((x, y, z)))
    }
}

