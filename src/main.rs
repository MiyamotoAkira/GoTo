extern crate goto;

use std::env;
use std::path;
use std::ffi::OsStr;
use goto::argument_parser::extract_arguments;
use goto::help::get_help;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let (result, arguments) = extract_arguments(args);

    if !result {
        let help = get_help();
        for line in help {
            println!("{}", line);
        }
        return Ok(());
    }

    let current = std::env::current_dir()?;
    let current_path = current.as_path();
    let mut possibles = Vec::new();

    visit_dirs(&current_path, &arguments.pattern, &mut possibles);

    println!("{:#?}", possibles);

    Ok(())
}

fn visit_dirs(dir: &path::Path, pattern: &String,  possibles: &mut Vec<String>) -> std::io::Result<()>{
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if  path.is_dir()  {
            let components = path.components();
            let final_entry = components.last();
            if final_entry == Some(path::Component::Normal(OsStr::new(pattern))) {
                let path_as_string = path.to_str().unwrap().to_string();
                possibles.push(path_as_string);
            }
            visit_dirs(&path, pattern, possibles);
        }
    }

    Ok(())
}
