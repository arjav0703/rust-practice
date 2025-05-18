// ALERT: This is not a Project
// This File containes my journey of learing Rust as a Typescript and Python developer.
// Things I like about Rust:
// 1. It is Fast
// 2. The compiler handles errors very well and even gives suggestions.
use rand::Rng;
use std::cmp::Ordering;
use std::io;
use std::{thread::sleep, time::Duration};
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

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    hostname: String,
    address: String,
}
fn main() {
    let _localhost = IpAddr {
        kind: IpAddrKind::V4,
        hostname: String::from("lonely-burrow"),
        address: String::from("192.168.1.110"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6,
        hostname: String::from("localhost"),
        address: String::from("::1"),
    };
    let _gateway = IpAddr {
        kind: IpAddrKind::V4,
        hostname: String::from("bbrouter"),
        address: String::from("192.168.1.1"),
    };
    println!(
        "My workstation's {:?} address is {} and it's hostname is {}",
        _localhost.kind, _localhost.address, _localhost.hostname
    );
    println!(
        "The Gateway's IP v4 is {} and it's hostname is {}",
        _gateway.address, _gateway.hostname
    );
}
