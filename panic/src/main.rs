use std::fs::File;

fn main() {


    // let f = File::open("I_DO_NOT_EXIST.txt").unwrap();


    // let f = File::open("I_DO_NOT_EXIST.txt").expect("Wah! does not exit!");


    let f = File::open("I_DO_NOT_EXIST.txt");

    let _foo = match f {
        Ok(file) => file,
        Err(err) => {
            panic!("File not found error!");
            // Or create the file or something...
        }
    };

}
