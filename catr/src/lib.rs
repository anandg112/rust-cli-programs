use clap::{App, Arg};
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
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
                .takes_value(false)
                .conflicts_with("number_nonblank"),
        )
        .arg(
            Arg::with_name("number_nonblank")
                .short("b")
                .long("number-nonblank")
                .help("Number non-blank lines")
                .takes_value(false),
        )
        .get_matches();

    Ok(Config {
        files: matches.values_of_lossy("files").unwrap(),
        // the two boolean options are present or not
        number_lines: matches.is_present("number"),
        number_nonblank_lines: matches.is_present("number_nonblank"),
    })
}

// function will accept a Config struct and return Ok with the unit type of successful
pub fn run(config: Config) -> MyResult<()> {
    for filename in config.files {
        // try to open the filename, borrowing the variable
        match open(&filename) {
            Err(err) => eprintln!("Failed to open {}: {}", filename, err),
            Ok(file_handle) => {
                for line_result in file_handle.lines() {
                    // unpack an Ok value from line_result or propagate an error
                    let line = line_result?;
                    println!("{}", line);
                }
            }
        }
    }
    Ok(())
}

//function will accept a filename and will return either an error or a boxed value that
// implements the Bufread trait
fn open(filename: &str) -> MyResult<Box<dyn BufRead>> {
    match filename {
        "-" => Ok(Box::new(BufReader::new(io::stdin()))),
        //using a Box to create a pointer to heap allocated memory to hold the filehandle
        _ => Ok(Box::new(BufReader::new(File::open(filename)?))),
    }
}
