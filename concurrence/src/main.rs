use std::{
    sync::mpsc,
    thread::{self, sleep},
    time::Duration,
};

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..=5 {
            println!("Second thread: {}", i);
        }
    });

    for i in 1..=3 {
        println!("First thread: {}", i);
    }
    handle.join().unwrap();

    let vector = vec![10, 20, 30];

    // vector foi movido para thread e não pertence mais a main.
    let handle = thread::spawn(move || {
        println!("{:?}", vector);
    });
    handle.join().unwrap();

    // Messaging passing
    let (tx, rx) = mpsc::channel();
    let tx2 = tx.clone();

    thread::spawn(move || {
        let values = vec![100, 200, 300];
        for v in values {
            sleep(Duration::from_secs(1));
            tx.send(v).unwrap();
        }
    });

    thread::spawn(move || {
        tx2.send(5_000).unwrap();
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
