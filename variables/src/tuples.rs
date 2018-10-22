fn main() {
    let tup: (i32, f64, u8) = (500, 3.5, 2);
    let (x, y, z) = tup;
    let b = tup.1;
    println!("X={}, Y={}, Z={}, B={}", x, y, z, b);
}
