use std::fs::File;

fn main() {
     let res = read_username_from_file("hello.txt".to_string());
        match res {
            Ok(s) => println!("The content of the file is: {}", s),
            Err(e) => println!("Error: {}", e),
        }
}

fn read_username_from_file(fileName: String) -> Result<String, std::io::Error> {
    let mut f = match File::open(fileName) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}