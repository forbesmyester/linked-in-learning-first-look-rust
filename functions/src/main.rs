fn main() {
    println!("Hello, world!");
    another_function();
    println!("ret = {}", sum_perhaps_plus_ten(31, 2));
    println!("loop = {}", make_greater_than_100(27));
    println!("sum = {}", sum_array([1, 2, 3, 4]));
}

fn another_function() {
    println!("ANOTHER");
}

fn sum_perhaps_plus_ten(x: i32, y: i32) -> i32 {
    let b = x + y;
    if b < 10 {
        return b + 10
    }
    return b;
}

fn make_greater_than_100(x: i32) -> i32 {
    let mut y: i32 = x;
    while y < 100 {
        y = y + 10
    }
    return y;
}

fn sum_array(ar: &[i32]) -> i32 {
    let mut r: i32 = 0;
    for x in ar.iter() {
        r = r + x;
    }
    return r;
}
