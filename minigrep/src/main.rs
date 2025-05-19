use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let to_find = &args[1];
    let file_path = &args[2];
    let content = read_file(to_find, file_path);
}

fn read_file(to_find: &str, file_path: &str) -> String{

    println!("[fn: read_file()] Looking for '{}' in '{}'", &to_find, &file_path);
    
    let content = fs::read_to_string(file_path).expect("Err reading file");
    return content;
}
