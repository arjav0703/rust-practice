use std::env;
use std::fs;

struct Config {
    to_find: String,
    file_path: String,
}

fn parse_config(args: &[String]) -> Config {
    let to_find = args[1].clone();
    let file_path = args[2].clone();

    Config{to_find, file_path}
}

fn read_file(to_find: &str, file_path: &str) -> String{

    println!("[fn: read_file()] Looking for '{}' in '{}'", &to_find, &file_path);
    
    let content = fs::read_to_string(file_path).expect("Err reading file");
    return content;
}


fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);
    let cfg = parse_config(&args);
    let content = read_file( &cfg.to_find, &cfg.file_path);
    println!("{}", content);
}

