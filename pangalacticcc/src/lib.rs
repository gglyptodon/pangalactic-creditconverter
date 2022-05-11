use clap::{Arg, Command};
use std::error::Error;
use std::fs::File;
use std::io::BufRead;

type PccResult<T> = Result<T, Box<dyn Error>>;

#[derive(Debug)]
pub struct Config {
    /// path to the input file with the gathered information.
    /// If path is "-", input will be read from stdin.
    path: String,
}

pub fn get_args() -> PccResult<Config> {
    let matches = Command::new("pangalacticc")
        .about("Pangalactic Credit Converter")
        .arg(
            Arg::new("input_path")
                .allow_invalid_utf8(true)
                .value_name("FILE")
                .default_value("-")
                .help(
                    "Input file with gathered information and queries. \
                    If set to '-' or no FILE is specified, input is read from stdin.",
                ),
        )
        .get_matches();

    // we can safely unwrap here because we set a default
    let path = matches.value_of_lossy("input_path").unwrap().to_string();

    Ok(Config { path })
}

/// output is printed to stdout
pub fn run(config: Config) -> PccResult<()> {
    //unimplemented!();
    let reader = open(&config.path)?;
    for line in reader.lines() {
        println!("{:?}", line);
    }

    Ok(())
}

pub fn open(path: &String) -> PccResult<Box<dyn BufRead>> {
    if path == "-" {
        // input is stdin
        Ok(Box::new(std::io::BufReader::new(std::io::stdin())))
    } else {
        // input read from file
        Ok(Box::new(std::io::BufReader::new(File::open(path)?)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests_run() {
        assert_eq!(1 + 2, 3);
    }
}
