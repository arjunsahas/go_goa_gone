#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
    let rect1 = Rectangle {width: 10, height: 20};
    println!("rect: {rect1:?}");

    dbg!(rect1);
}
