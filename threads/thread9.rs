use std::thread;

fn main() {
    let mut n = 1;

    let t = thread::spawn(move || {
        n = n + 1;
        println!("{} from thread1", n);

        thread::spawn(move || {
            n = n + 1;
            println!("{} from thread2", n);
        })
    });

    n = n + 1;
    print!("{} from thread3", n);

    t.join().unwrap().join().unwrap();

    println!("{n}");
}