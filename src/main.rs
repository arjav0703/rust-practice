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

    let temp: u32 = 30;
    if temperature <= 10 {
        println!("The weather is cold");
    } else if temperature <= 20 {
        println!("The weather is cool");
    } else {
        println!("The weather is warm");
    }
}
