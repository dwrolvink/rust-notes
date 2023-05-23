use thiserror; 
/*
    // add this to cargo.toml:
    [dependencies]
    thiserror = "1.0"
*/

// create an enum of valid error types
// annotate each to make the error printable
#[derive(thiserror::Error, Debug, Clone)]
pub enum Error {
    #[error("Internal error.")]
    Internal(String),
    #[error("Not found.")]
    NotFound,
    #[error("Run failed. Reason: {0}")]
    RunFailed(String),
    #[error("Invalid argument: {0}")]
    InvalidArgument(String),
}

// function that returns simple Result
pub fn ultimate_answer(guess: i64) -> Result<String, Error> {
    if guess == 42 {
      return Ok("done".to_string());
    }
    return Err(Error::RunFailed("bork".to_string()));
  }

pub fn main() {
    // only allow OK, otherwise panic
    let res = ultimate_answer(42).unwrap();
    println!("{}", res);

    // continue on error, don't panic
    match ultimate_answer(12) {
        Ok(description) => println!("{}", description),
        Err(err) => println!("Error: {}", err),
    }
}