use std::{thread::sleep, time::Duration};

fn main() {
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
