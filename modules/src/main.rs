use garden::vegetables::{self};
use modules::front_of_house::hosting;

mod garden;

fn main() {
    let asparagus = vegetables::Asparagus {
        width: 10,
        height: 20,
        plants: String::from("Asparagus"),
    };
    hosting::add_to_waitlist();
    println!("Asparagus width: {} - {} - {}", asparagus.width, asparagus.height, asparagus.plants);
}
