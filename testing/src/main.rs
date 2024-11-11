fn main() {
    let tup = (500, 6.4, 1, 'a', true);
    let (x, y, z, a, b) = tup;
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    println!("The value of z is: {}", z);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    let five_hundred = tup.0;
    let six_point_four = tup.1;
    let one = tup.2;
    let a = tup.3;
    let b = tup.4;
    println!("The value of five_hundred is: {}", five_hundred);
    println!("The value of six_point_four is: {}", six_point_four);
    println!("The value of one is: {}", one);
    println!("The value of a is: {}", a);
    println!("The value of b is: {}", b);
    let mut arr: [i32; 5] = [1, 2, 3, 4, 5];
    let first = arr[0];
    let second = arr[1];
    println!("The value of first is: {}", first);
    println!("The value of second is: {}", second);
    arr[0] = 6;
    let new_first = arr[0];
    println!("The value of new_first is: {}", new_first);
}
