use std::sync::mpsc; // use multiple producer single consumer channel
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel::<String>(); // create channel
    let vec_var = vec![String::from("hello"), String::from("there")];

    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        for v in vec_var.iter() {
            tx.send(v.to_string()).unwrap(); // send through transmitter
        }
    });

    let vec_var1 = vec![String::from("this is "), String::from("rust")];

    thread::spawn(move || {
        for v in vec_var1.iter() {
            tx1.send(v.to_string()).unwrap(); // send through transmitter
        }
    });

    for r in rx { // receive message through receiver. We consider rx as iterator.
        println!("{}", r);
    }
}