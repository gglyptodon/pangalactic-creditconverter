use clap::{Arg, Command};
use std::error::Error;

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
    println!("{:?}", config);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tests_run() {
        assert_eq!(1 + 2, 3);
    }
}
