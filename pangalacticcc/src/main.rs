use pangalacticcc::{get_args, run};

fn main() {
    if let Err(e) = get_args().and_then(run) {
        // if an error occurs, print out the error message to stderr
        // and quit with non-zero exit code
        eprintln!("{}", e);
        std::process::exit(1)
    }
}
