use std::fs::File;
use std::io::ErrorKind;


fn main() {
    let _greeting_file = match File::open("hello.txt") {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(file) => panic!("Here"),
                Err(error) => {
                    panic!("Problem creating the file: {:?}", error)
                },
            },
            _ => {
                panic!("Problem opening the file: {:?}", error)
            },
        },
    };
}
