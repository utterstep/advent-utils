use std::{error::Error, str::FromStr};

use crate::{get_config, read_file, Part};

pub trait Solver {
    fn solve(&self, part: Part) -> String;

    fn day_number() -> u32;

    fn solve_env_config() -> Result<(), Box<dyn Error>>
    where
        Self: FromStr<Err = Box<dyn Error>>,
    {
        let config = get_config()?;
        let solution: Self = read_file(config.input_file)?.parse()?;

        println!("{}", solution.solve(config.part));

        Ok(())
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One, Part::Two]
    }
}
