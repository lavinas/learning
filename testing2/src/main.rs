// This program demonstrates how to create a struct and use it in Rust

// Define a struct User with four fields
fn main() {
    let user1 = User {
        active: true,
        username: String::from("user1"),
        email: String::from("x@test.com"),
        sign_in_count: 1,
    };
    println!("user1: {}, {}, {}, {}", user1.active, user1.username, user1.email, user1.sign_in_count);
    let user2 = User {
        active: user1.active,
        username: user1.username,
        email: user1.email,
        sign_in_count: user1.sign_in_count,
    };
    println!("user2: {}, {}, {}, {}", user2.active, user2.username, user2.email, user2.sign_in_count);
    let user3 = User {
        ..user2
    };
    println!("user3: {}, {}, {}, {}", user3.active, user3.username, user3.email, user3.sign_in_count);
    let c = Color(255, 0, 0);
    let p = Point(0, 0, 0);
    println!("color: {}, {}, {}", c.0, c.1, c.2);
    println!("point: {}, {}, {}", p.0, p.1, p.2);
    println!("area: {}", area(30, 40));
    println!("area2: {}", area2((30, 40)));
    let rect1 = Rectangle {
        width: 30,
        height: 40,
    };
    println!("area3: {}", area3(&rect1));
    println!("rect1: {:#?}", rect1);
    dbg!(rect1);
    let rect2 = Rectangle2 {
        width: 20,
        height: 30,
    };
    println!("area4: {}", rect2.area());
}

// User struct is defined here
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// create a tuple struct called Color
struct Color(i32, i32, i32);

// create a tuple struct called Point
struct Point(i32, i32, i32);

// create a function called area that takes two u32 arguments and returns a u32 value
fn area(witdh: u32, height: u32) -> u32 {
    witdh * height
}

// create a function area2 that takes a reference a tuple with two u32 values and returns a u32 value
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}


// create a function called area3 that takes a reference to a struct Rectangle and returns a u32 value
fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}

// create a struct called Rectangle with two fields
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Rectangle2 {
    width: u32,
    height: u32,
}

impl Rectangle2 {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}