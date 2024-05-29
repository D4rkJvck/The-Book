use std::{thread, time::Duration};

fn main() {
    let mut handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    handle.join().unwrap();

    for i in 1..5 {
        println!("hi number {i} from main thread");
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    handle = thread::spawn(move || {
        println!("Here is a vector: {:?}", v);
        thread::sleep(Duration::from_millis(1));
    });

    handle.join().unwrap();
}
