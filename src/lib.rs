#[cfg(test)]
#[macro_use(quickcheck)]
extern crate quickcheck_macros;

mod file_load;
pub use crate::file_load::{parse_file, parse_raw_data, read_file};

#[cfg(feature = "argparse")]
mod run_config;
#[cfg(feature = "argparse")]
pub use crate::run_config::{get_config, get_custom_config, Config, Part};

#[cfg(feature = "nom-macro")]
mod nom_macros;

pub mod algo;

mod traits;
pub use traits::Solver;
