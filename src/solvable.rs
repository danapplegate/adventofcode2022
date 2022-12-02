use std::error::Error;
use std::path::PathBuf;

pub trait Solvable<const N: usize> {
    fn new(filename: &str) -> Self;
    fn data_path() -> PathBuf {
        format!("data/{}", N).into()
    }

    fn answer1(&self) -> Result<String, Box<dyn Error>>;
    fn answer2(&self) -> Result<String, Box<dyn Error>>;
}
