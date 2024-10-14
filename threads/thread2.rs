use std::thread;
use std::time::Duration;
fn main() {
    let t1 = thread::spawn(|| {
        for i in 1..10
        {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i1 in 1..5 {
        println!("hi number {} from the main thread!", i1);
        thread::sleep(Duration::from_millis(1));
    }

    t1.join().unwrap(); // main thread waits for the spawned thread.

}