use std::ops::Add;

// #[derive(Debug, Copy, Clone, PartialEq)]
#[derive(PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn main() {

    // assert_eq!(
    //    Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
    //    Point { x: 3, y: 3 }
    // );
    let p1 = Point { x: 1, y: 0 };
    let p2 = Point { x: 2, y: 3 };
    let p3 = p1 + p2;
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
    let p4 = Point { x: 3, y: 3 };
    let p5 = Point { x: 3, y: 3 };
    if p4 == p5 {
        println!("p1 == p2");
    } else {
        println!("p1 != p2");
    }
}