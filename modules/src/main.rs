use garden::vegetables::{self};
use modules::front_of_house::hosting;
use rand::Rng;

mod garden;

fn main() {
    let asparagus = vegetables::Asparagus {
        width: 10,
        height: 20,
        plants: String::from("Asparagus"),
    };
    hosting::add_to_waitlist();
    println!("Asparagus width: {} - {} - {}", asparagus.width, asparagus.height, asparagus.plants);
    // other
    let secret_number = rand::thread_rng().gen_range(1..101);
    println!("The secret number is: {}", secret_number);
}
