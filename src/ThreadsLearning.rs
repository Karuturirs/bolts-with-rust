use std::thread;
use std::time::Duration;

fn main() {

    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    let v = vec![1, 2, 3];

    let handler = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    //drop(v); // value of v has been moved with move in closures
    handler.join().unwrap();
    handle.join().unwrap();
}
