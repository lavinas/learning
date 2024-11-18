fn main() {
    // The Vec<T> type is a growable array type, implemented using a Vec<T> struct.
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    println!("{:?}", v);
    // create a vector of integers with 1, 2, 3
    let v = vec![1, 2, 3];
    println!("{:?}", v);
    // create a vector of strings
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("Hello, "));
    v.push(String::from("World!"));
    println!("{:?}", v);
    // getting the third element of a vector of strings
    let v = vec![String::from("a"), String::from("b"), String::from("c")];
    let third: &String = &v[2];
    println!("The third element is {}", third);
    // getting non-existent element of a vector of strings
    // let does_not_exist = &v[100]; // panic
    let v: Option<&String> = v.get(100);
    match v {
        Some(value) => println!("The value is: {}", value),
        None => println!("There is no value."),
    }
    // create a vector of enums with different types
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    let first = &row[0];
    let int1 = match first {
        SpreadsheetCell::Int(value) => value,
        _ => &0,
    };
    println!("{}", int1);
    let text1 = match &row[1] {
        SpreadsheetCell::Text(value) => value,
        _ => &String::from(""),
    };
    println!("{}", text1);
    let float1 = match &row[2] {
        SpreadsheetCell::Float(value) => value,
        _ => &0.0,
    };
    println!("{}", float1);

}    