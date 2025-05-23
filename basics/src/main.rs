use news::{ Formatter, NewsArticle};
// ALERT: This is not a Project
// This File containes my journey of learing Rust as a Typescript and Python developer.
// Things I like about Rust:
// 1. It is Fast
// 2. The compiler handles errors very well and even gives suggestions.
use rand::Rng;
use core::panic;
use std::cmp::Ordering;
use std::io::ErrorKind;
use std::{i32, io};
use std::{thread::sleep, time::Duration};
use std::collections::HashMap;
use std::fs::File;
mod news;
// mod garden;
// #[derive(Debug)]
// struct Usr {
//     active: bool,
//     email: String,
//     usrname: String,
//     sign_in_count: u64,
// }

// impl Usr {
//     fn _new(email: String, usrname: String, active: bool, sign_in_count: u64) -> Usr {
//         Usr {
//             active,
//             email,
//             usrname,
//             sign_in_count,
//         }
//     }
// }

fn _something() {
    println!("Hellow World!");

    let _x: i32 = -50000000; // 32 bit one can't store much value
    let _y: i64 = -50000000000000;

    // tuples: fix in length =================
    let cords: (char, i32) = ('a', 4);
    let _foo = cords.1;
    // Destructuring
    let (_c1, _c2) = cords;

    //===== ARRAYS ========
    let _arr = ["3", "1", "6", "223", "43", "2"];
    let _item1 = _arr[0];

    // Control Flow
    let temp: u32 = 30;

    let weather = if temp <= 10 {
        "cold"
    } else if temp <= 20 {
        "pleasing"
    } else {
        "hot"
    };

    println!("The weather is {}", weather);

    // ==== LOOP ========
    let mut counter = 0;
    // Infinite loop
    loop {
        println!("Charging....{}%", counter);
        counter += 10;
        sleep(Duration::from_secs(1));
        if counter >= 100 {
            eprintln!("Battery reached {}%", counter);
            break;
        }
    }
}

// Func ===== the "->" represents the type the function will return
fn _add(a: i32, b: i32) -> i32 {
    // a + b absense of semicolon means return
    // OR
    return a + b; // this is also valid
}

fn _ownership() {
    let str1 = String::from("Hellow! I am a cat 😸");
    let str2 = str1; // ownership of str1 is moved to str2

    println!("str2 is '{}'", str2);
    // println!("str1 is {}", str1); // this will throw an error
}

fn _reference() {
    let mut _str1 = String::from("Hellow! I am a cat 😸");
    let mut _str2 = &_str1[0..6]; // this is a slice
    println!("{_str2}");
    _str2 = "Hellow! world"; // this is a mutable slice

    println!("str2 is '{}' while str1 is {}", _str2, _str1);
}
fn _greet() {
    let mut usrinput = String::new();
    println!("Hey Rustacean 🦀, what's your name? ");

    io::stdin()
        .read_line(&mut usrinput)
        .expect("Failed to read line");
    println!("Hey {}", usrinput);
}

fn _guess() {
    println!("Guess the number!");
    loop {
        let secret_number = rand::thread_rng().gen_range(1..=10);

        println!("The secret number is: {}", &secret_number);

        println!("Please input your guess: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                eprintln!("Please enter a number!");
                continue;
            }
        };

        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}

fn _vars() {
    let x = 20;
    let y = &x + 10;

    {
        // Shadowing
        let x = 50;
        println!("[Inner Scope] X is {}", x)
    }
    print!("X is {} \nY is {}", &x, &y);
}

fn _arr() {
    let months: [&str; 12] = [
        "jan", "feb", "mar", "apr", "may", "jun", "jul", "aug", "sep", "oct", "nov", "dec",
    ];
    println!("Which Month Do you want to know about?");
    let mut ind = String::new();

    io::stdin()
        .read_line(&mut ind)
        .expect("Failed to read line");

    let ind: i32 = ind.trim().parse().expect("Please enter a number!");

    if !(ind <= 0 || ind > 12) {
        println!("The month is {}", months[(ind - 1) as usize]);
    } else {
        println!("Please enter a number between 1 and 12");
    }
}

fn _loop() {
    for num in (1..10).rev() {
        println!("The number is {}", num);
    }
    println!("LIFTOFF")
}

fn _temp() {
    let mut res = String::new();
    println!("Please enter a temperature in Celsius: ");
    io::stdin()
        .read_line(&mut res)
        .expect("Failed to read line");
    let mut res: f32 = res.trim().parse().expect("Please enter a number!");

    res = (res * 9.0 / 5.0) + 32.0;

    println!("{}F", res)
}

fn _slice(s: &String) -> String {
    let parts: Vec<&str> = s.split(' ').collect(); // Vec<&str> tells .collect() what to return
    String::from(parts[0])
}

// #[derive(Debug)]
// enum IpAddrKind {
    // V4,
    // V6,
// }

// struct IpAddr {
    // kind: IpAddrKind,
    // hostname: String,
    // address: String,
// }

fn _some() {
    // Rust does not have null. Instead it has Option<T> where T is the type of value
    // let nullvalue: Option<i8> = None;

    //let x: i8 = 5;
    //let y: Option<i8> = Some(5);

    //let sum = x + y;
    println!("This will error out as we cant add i8 and Option<i8>");
}

fn _vectors() {
    let mut vector1: Vec<i32> = Vec::new();
    vector1.push(6);
    vector1.push(9);

    println!("Out vector is : {:?}", vector1);

    let _third: Option<&i32> = vector1.get(2);
    match _third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element"),
    }
    //println!("The third element is {:?}", _third);
}

fn _hashmap() {
    // HashMap is a key-value pair
    // HashMap(key , value)
    let mut scores = HashMap::new();
    scores.insert(String::from("blue"), 10);
    scores.insert(String::from("yellow"), 50);
    scores.insert(String::from("red"), 20);
    scores.insert(String::from("green"), 30);
    
    println!("Enter Team name to get scores");
    
    let mut key = String::new();
    io::stdin().read_line(&mut key).expect("Failed to read line");
    let key = key.trim().to_lowercase();
    
    let res = match scores.get(&key) {
        Some(&score) => score,
        None => 0,
    };

    println!("The score is {}", res);

    scores.insert(String::from("blue"), 50);
}

fn _challenge() {
    let mut vec: Vec<i32> = vec![1, 2, 3, 4, 5, 2, 4, 1, 1, 6 , 1, 64, 2, 6, 1, 4];
    // println!("The vector is {:?}", &vec);
    vec.sort();
    // println!("The sorted vector is {:?}", vec);
    let med = _find_median(&mut vec);
    println!("The median is {}", med);
}

fn _find_mode(numbers: &Vec<i32>)  {
    let mut map = HashMap::new();

    for num in numbers {
        let counter = map.entry(num).or_insert(0);
        *counter += 1;
    }
    println!("HashMap: {:?}", map);
    
    let mut max_k = None;
    let mut max_v = i32::MIN;

    for (&key, &value) in &map {
        if value > max_v {
            max_v = value;
            max_k = Some(key);
        }        
    }
    match max_k {
        Some(k) => println!("The mode is {} with {} occurences", k, max_v),
        None => println!("No mode found"),
    }
}

fn _find_median(numbers: &mut Vec<i32>) -> f64 {
    _find_mode(numbers);

    let len = numbers.len();
    if len % 2 == 0 {
        // If even, return the average of the two middle elements
        let mid1 = numbers[len / 2 - 1];
        let mid2 = numbers[len / 2];
        (mid1 + mid2) as f64 / 2.0
    } else {
        // If odd, return the middle element
        numbers[len / 2] as f64
    }

}

fn _err_handling(){
    let filename = "hello.txt";
    let file = File::open(filename);
    let file = match file {
        Ok(file) => file,
        Err(err) => match err.kind(){
            ErrorKind::NotFound => {
                println!("{} does not exist", filename );
                match File::create(filename) {
                    Ok(fc) => {
                        println!("File created successfully: {:?}", fc);
                    }
                    Err(err) => {
                        panic!("Unable to create {:?} due to: {} ", filename, err )
                    }
                }
                return;
            }
        _ => {        
                // eprintln!("Error opening file: {}", err);
                panic!("{} not found: {:?}", filename, err);
            }
        }
    };
    println!("File opened successfully: {:?}", file);
}

// fn _largest<T>(list: &[T]) -> &T{ // T is a generic type
    // let mut res = &list[0];

    // for item in list {
        // if item > res {
            // res = item;
        // }
    // }
    // res
// }


fn main() {
    let a = NewsArticle{
        title: String::from("Rust is awesome"),
        content: String::from("Rust is a systems programming language that is blazingly fast and memory-efficient due to it's ownership model."),
        author: String::from("Arjav Jain"),
    };
    println!("{}", a.format());

}
