use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    // dbg!(&args);

    let to_find = &args[1];
    let file_path = &args[2];

    println!("info Looking for '{}' in '{}'", &to_find, &file_path);
}
