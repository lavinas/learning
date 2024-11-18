use std::{fs::File, io::Read};

fn main() {
    let res = read_username_from_file("hello.txt".to_string());
    match res {
        Ok(s) => println!("The content of the file is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
    let res = read_username_from_file_2("hello.txt".to_string());
    match res {
        Ok(s) => println!("The content of the file is: {}", s),
        Err(e) => println!("Error: {}", e),
    }
}

fn read_username_from_file(file_name: String) -> Result<String, std::io::Error> {
    let mut username_file = match File::open(file_name) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
   
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_2(file_name: String) -> Result<String, std::io::Error> {
    let mut username = String::new();
    File::open(file_name)?.read_to_string(&mut username)?;
    Ok(username)
}