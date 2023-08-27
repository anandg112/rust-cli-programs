use clap::{App, Arg};
use std::error::Error;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)] //derive macro adds the debug trait so the struct can be printed.
pub struct Config {
    files: Vec<String>,          //files will be a vector of string
    number_lines: bool,          //bool value to indicate whether or not to prin the line numbers
    number_nonblank_lines: bool, //bool value to control printing line numbers only for non-blank lines
}

pub fn get_args() -> MyResult<Config> {
    let matches = App::new("catr")
        .version("0.1.0")
        .author("Anand Gautam")
        .about("Rust cat")
        .arg(
            Arg::with_name("files")
                .value_name("FILE")
                .help("Input file(s)")
                .multiple(true)
                .default_value("-"),
        )
        .arg(
            Arg::with_name("number")
                .short("n")
                .long("number")
                .help("Number lines")
                .takes_value(false),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help(" Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// function will accept a Config struct and return Ok with the unit type of successful
pub fn run(config: Config) -> MyResult<()> {
    dbg!(config); //use the debug macro to print the configuration
    Ok(())
}
