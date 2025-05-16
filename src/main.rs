use std::io;
use std::{thread::sleep, time::Duration};

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
fn main() {
    // // let sum = add(2, 4);
    // println!("The sum is {}", add(2, 4));
    _greet();
}
