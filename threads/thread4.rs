use std::sync::mpsc; // use multiple producer single consumer channel
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel::<String>(); // create channel

    thread::spawn(move || {
        tx.send(String::from("Hello, World!")).unwrap(); // send through transmitter
    });

    let rec = rx.recv().unwrap(); // receive message through receiver.
    println!("{}", rec);
}