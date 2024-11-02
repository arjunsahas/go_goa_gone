use std::sync::{Arc, Mutex};
use std::thread;
///
fn main() {
    let count = Arc::new(Mutex::new(0));
    let mut thread_handles = vec![];
    let mut n = 1;

    for _ in 0..10 {
        let counter = Arc::clone(&count);
        let thread_handle = thread::spawn(move || {
            n = n + 1;
            let mut c = counter.lock().unwrap();
            *c += 1;
        });
        thread_handles.push(thread_handle);
    }

    for handle in thread_handles {
        handle.join().unwrap();
    }

    println!("Count: {}", *count.lock().unwrap());
}