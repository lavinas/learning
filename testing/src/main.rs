fn main() {
    let s = gives_ownership();
    println!("s: {}", s);
    let s2 = takes_and_gives_back(s);
    println!("s3: {}", s2);
    let (s3, len) = calculate_length(s2);
    println!("s3: {}, len: {}", s3, len);
    let mut s4 = String::from("hello");
    let len = calculate_length_ref(&mut s4);
    println!("s4: {}, len: {}", s4, len);
    let s5 = &s4;
    println!("s5: {}", s5);
    let s6 = &mut s4;
    println!("s6: {}", s6);
    let s7 = &s4;
    println!("s7: {}", s7);
    let s8 = String::from("hello world");
    let s9= &s8[0..5];
    println!("s9: {}", s9);
    let (s10, s11) = s8.split_at(5);
    println!("s10: {}, s11: {}", s10, s11);
    let s11 = "hello world";
    let s12 = &s11[0..5];
    println!("s12: {}", s12);
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    println!("slice: {:?}", slice);
    let b = (1, 2, 3, 4, "a");
    let slice2 = &b.1..&b.3;
    println!("slice2: {:?}", slice2);
    
}

fn gives_ownership() -> String {
    let s = String::from("hello");
    s
}

fn takes_and_gives_back(s: String) -> String {
    s
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    (s, length)
}

fn calculate_length_ref(s: &mut String) -> usize {
    s.push_str(" world");
    s.len()
}