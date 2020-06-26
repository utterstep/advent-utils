use std::{convert::TryFrom, error::Error};

use crate::{get_config, Part};

pub trait Solver: TryFrom<String, Error = Box<dyn Error>> {
    fn solve(&self, part: Part) -> String;

    fn day_number() -> u32;

    fn solve_env_config() -> Result<(), Box<dyn Error>> {
        let config = get_config()?;
        let solution = Self::try_from(config.input_file)?;

        println!("{}", solution.solve(config.part));

        Ok(())
    }

    fn implemented_parts() -> Vec<Part> {
        vec![Part::One, Part::Two]
    }
}
