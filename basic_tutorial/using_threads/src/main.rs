use std::{thread, time::Duration};

fn main() {
    parallel();
    joined();
}

fn parallel() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Subthread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });
    for i in 1..5 {
        println!("Main thread : {}", i);
        thread::sleep(Duration::from_millis(1));
    }
    
    handle.join().unwrap();
}

fn joined() {
    let v = vec![1,2,3];

    let handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
    });

    handle.join().unwrap();
}