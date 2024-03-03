use std::env;
use std::fs;

fn grep_from_file(file_path: &str, query: &str) {
    let content = fs::read_to_string(file_path)
        .expect("Should have been able to read the file");

    for line in content.lines() {
        if line.contains(query) {
            println!("{}", line);
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    dbg!(&args);
    match (args.get(1).map(String::as_str), args.get(2).map(String::as_str), args.get(3).map(String::as_str)) {
        (Some("grep"), Some("-f"), Some(file)) => grep_from_file(&args[2], file),
        _ => println!("Invalid arguments"),
    }
}

