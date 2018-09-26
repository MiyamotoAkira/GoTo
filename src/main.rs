extern crate goto;

use std::env;
use goto::argument_parser::extract_arguments;
use goto::help::get_help;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (result, arguments) = extract_arguments(args);

    if !result {
        let help = get_help();
        for line in help {
            println!("{}", line);
        }
    }
}
