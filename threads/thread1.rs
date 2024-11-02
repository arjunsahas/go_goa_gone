use std::thread;
use std::time::Duration;
fn main() {

    // rust follows 1:1 thread model
    thread::spawn(|| {
        for i in 1..10
        {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // the main thread does not wait for the spawned thread and the spawned thread prematurely exits.
    for i1 in 1..5 {
        println!("hi number {} from the main thread!", i1);
        thread::sleep(Duration::from_millis(1));
    }
}