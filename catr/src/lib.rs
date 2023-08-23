use std::error::Error;

#[derive(Debug)] //derive macro adds the debug trait so the struct can be printed.
pub struct Config {
    files: Vec<String>,
    number_lines: bool,
    number_nonblank_lines: bool,
}

type MyResult<T> = Result<T, Box<dyn Error>>;

pub fn run() -> MyResult<()> {
    println!("Hello World!");
    Ok(())
}