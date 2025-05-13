fn main() {
    println!("Hellow World!");

    let _x: i32 = -50000000; // 32 bit one can't store much value
    let _y: i64 = -50000000000000;

    // tuples: fix in length
    let cords: (char, i32) = ('a', 4);
    let _foo = cords.1 
    // Destructuring
    let (_c1, _c2) = cords;
}
