fn main() {
    let v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter();
    let res = match v1_iter.next() {
        Some(value) => value,
        None => &0,
    };
    println!("[0] {:?}", res);
    // sum
    let sum1: i32 = v1_iter.sum();
    println!("sum {:?}", sum1);
    // map
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();
    println!("v2 {:?}", v2);
    // filter
    let v3: Vec<_> = v1.iter().filter(|x| *x % 2 == 0).collect();
    println!("v3 {:?}", v3);
}
