fn main() {
    // let refers_nothing = dangle();
    let refers_something = no_dangle();
    println!("{}", refers_something);
}

// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s;
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s
}
