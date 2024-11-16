enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),    
}


#[derive(Debug)] // so we can inspect the state in a minute
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

impl Coin {
    fn value_in_cents(&self) -> u8 {
        match self {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            },
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                match state {
                    UsState::Alabama => 25,
                    UsState::Alaska => 26,
                }
            },
        }
    }
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move: {}, {}", x, y),
            Message::Write(s) => println!("Write: {}", s),
            Message::ChangeColor(r, g, b) => println!("ChangeColor: {}, {}, {}", r, g, b),
        }
    }
}

fn main() {
    let m1 = Message::Quit;
    m1.call();
    let m2 = Message::Move { x: 10, y: 20 };
    m2.call();
    let m3 = Message::Write(String::from("Hello"));
    m3.call();
    let m4 = Message::ChangeColor(255, 0, 0);
    m4.call();
    let _some_number = Some(5);
    let _some_char = Some('e');
    let _absent_number: Option<i32> = None;
    // check coin value
    let coin = Coin::Quarter(UsState::Alabama);
    println!("coin value: {}", coin.value_in_cents());
    let coin = Coin::Quarter(UsState::Alaska);
    println!("coin value: {}", coin.value_in_cents());
    let coin = Coin::Penny;
    println!("coin value: {}", coin.value_in_cents());
    let coin = Coin::Nickel;
    println!("coin value: {}", coin.value_in_cents());
    let coin = Coin::Dime;
    println!("coin value: {}", coin.value_in_cents());
    // other
    let config_max: Option<i8> = Some(8);
    if let Some(m) = config_max {
        println!("The maximum is configured to be {m}");
    }
    match config_max {
        Some(max) => println!("The maximum is configured to be {max}"),
        None => println!("No maximum configured"),
    };
    // if let
    let coin = Coin::Penny;
    if let Coin::Penny = coin {
        println!("Lucky penny!");
    };
    match coin {
        Coin::Penny => println!("Lucky penny 2!"),
        _ => (),
    };

}
