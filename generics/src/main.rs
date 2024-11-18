fn largest<T: std::cmp::PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list.iter() {
        if item > largest {
            largest = &item;
        }
    }
    largest
}

struct Point<T, W> {
    x: T,
    y: W,
}

impl Point<i32, i32> {
    fn xy(&self) -> &i32 {
        &self.x
    }
}

fn main() {
    let number_list = vec![1000, 34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
    // other
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    let p = Point { x: 5.0, y: 10.0 };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    let p: Point<i32, String> = Point { x: 5, y: "BBB".to_string() };
    println!("p.x = {}", p.x);
    println!("p.y = {}", p.y);
    // other
    let res = Result::Ok::<i32, String>(5);
    match res {
        Result::Ok(v) => println!("Ok: {}", v),
        Result::Err(e) => println!("Err: {}", e),
    }
    let opt = Option::Some(5);
    match opt {
        Option::Some(v) => println!("Some: {}", v),
        Option::None => println!("None"),
    };
    let opt: Option<i32> = Option::None;
    match opt {
        Option::Some(v) => println!("Some: {}", v),
        Option::None => println!("None"),
    };
    // other
    let p = Point { x: 5, y: 10 };
    println!("p.x = {}", p.xy());
    // let p = Point { x: 5.0, y: 10.0 };
    // println!("p.x = {}", p.xy());
}
