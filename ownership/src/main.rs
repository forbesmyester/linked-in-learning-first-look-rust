fn main() {
    let mut s1 = String::from("hello");
    takes_ownership(&mut s1);
    println!("{}", s1);
}

fn takes_ownership(s: &mut String) {
    println!("{}", s);
    s.push_str(" World!");
}
