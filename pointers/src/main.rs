struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

use std::ops::Deref;
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use std::ops::DerefMut;
impl<T> DerefMut for MyBox<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_mut(name: &mut String) {
    name.push_str(" mut ");
    println!("Hello, {}!", name);
}

struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

use std::rc::Rc;
#[derive(Debug)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

fn main() {
    // box
    let b = Box::new(5);
    println!("b = {b}");
    // mybox
    let x = 5;
    let y = MyBox::new(x);
    assert_eq!(5, x);
    assert_eq!(5, *y);
    // deref coercion
    let mut m = MyBox::new(String::from("Rust"));
    hello(&m);
    m = MyBox::new(String::from("XXX"));
    hello(&m);
    // deref coercion with mutable reference
    let mut m = MyBox::new(String::from("Rust"));
    hello_mut(&mut m);
    println!("m = {}", *m);
    // drop
    let c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    println!("c = {}", c.data);
    // drop with early exit
    let c = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("c = {}", c.data);
    drop(c);
    println!("CustomSmartPointer dropped before the end of main.");
    // Rc
    use crate::List::{Cons, Nil};
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    println!("count after creating b{:?} = {}", b, Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c{:?} = {}", c, Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

}
