use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::time::Duration;

fn main() {
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
    println!("Result: {}", *counter.lock().unwrap());
}

fn _main0() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {i} from the spawned thread!");
            thread::sleep(Duration::from_millis(10));
        }
    });
    for i in 1..5 {
        println!("hi number {i} from the main thread!");
        thread::sleep(Duration::from_millis(1));
    }
    handle.join().unwrap();
}

fn _main1() {
    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vector: {v:?}");
    });
    handle.join().unwrap();
}

fn _main2() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        println!("val is: {val}");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got: {received}");
}

fn _main3() {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];
        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
    for received in rx {
        println!("Got: {received}");
    }
}

fn _main4() {
    let (tx, rx) = mpsc::channel();
    let tx1 = tx.clone();
    _send_message(vec![
        String::from("hi"),
        String::from("from"),
        String::from("the"),
        String::from("thread"),
    ], tx);
    _send_message(vec![
        String::from("more"),
        String::from("messages"),
        String::from("for"),
        String::from("you"),
    ], tx1);
    for received in rx {
        println!("Got: {received}");
    }
}

fn _send_message(messages: Vec<String>, tx: mpsc::Sender<String>) {
    thread::spawn(move || {
        for val in messages {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });
}