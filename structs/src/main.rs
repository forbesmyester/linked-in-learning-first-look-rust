enum EmployeeType {
    Perm(String),
    Temp(String),
}

impl EmployeeType {
    fn to_string(&self) -> String {
        match self {
            EmployeeType::Temp(s) => String::new() + "Temp: " + s,
            EmployeeType::Perm(s) => String::new() + "Perm: " + s,
        }
    }
    fn makePerm(self) -> EmployeeType {
        match self {
            EmployeeType::Temp(s) => EmployeeType::Perm(s),
            _ => self
        }
    }
}

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.height * self.width
    }
}

struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
    empType: EmployeeType,
}

fn main() {
    println!("Hello, {}", build_user("Matt".to_string(), "a@b".to_string()).username);
    let rect1 = Rectangle { width: 10, height: 20 };
    println!("Area = {}", area(&rect1));
    println!("Width = {}", rect1.width);
    let bob = build_user("Bob".to_string(), "a@b".to_string());
    println!("Bye, {} {}", bob.username, bob.empType.makePerm().to_string());
}

fn build_user(username: String, email: String) -> User {
    User {
        email,
        username,
        sign_in_count: 0,
        active: true,
        empType: EmployeeType::Temp(String::from("Coder"))
    }
}

fn area(rect: &Rectangle) -> u32 {
    rect.area()
}
