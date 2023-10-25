use std::env;
use std::ffi::OsStr;
use std::fs;
use std::io;
use std::io::IsTerminal;
use std::io::Read;
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut stdio = io::stdin();
    let mut data = String::new();
    let mut command = "all";
    let mut path = Path::new("");

    if args.len() == 1 && stdio.is_terminal() {
        print!("Provide some args");
        return;
    }

    if stdio.is_terminal() && args.len() == 2 {
        path = Path::new(&args[1]);
    } else if args.len() > 2 {
        command = &args[1];
        path = Path::new(&args[2]);
    } else if !stdio.is_terminal() && args.len() > 1 {
        command = &args[1];
    }

    if !stdio.is_terminal() {
        stdio
            .read_to_string(&mut data)
            .expect("error has occured when getting data from pipe");
    } else {
        data = fs::read_to_string(path).expect("error has occured when reading from a file");
    }

    let filename = path
        .file_name()
        .unwrap_or_else(|| OsStr::new(""))
        .to_str()
        .expect("error when getting filename");

    match command {
        "-c" => {
            println!("  {:?} {}", data.len(), filename);
        }
        "-l" => {
            let lines  = data.lines().count();
            println!("  {:?} {}", lines, filename);
        }
        "-w" => {
            let words = data.split_whitespace().count();
            println!("  {:?} {}", words, filename);
        }
        "-m" => {
            let chars = data.chars().count();
            println!("  {:?} {}", chars, filename);
        }
        "all" => {
            let lines  = data.lines().count();
            let words = data.split_whitespace().count();
            println!(
                "  {:?} {:?} {:?} {}",
                lines,
                words,
                data.len(),
                filename
            )
        }
        _ => println!("Unknown Argument"),
    }
}
