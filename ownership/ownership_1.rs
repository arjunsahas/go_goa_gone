fn main() {
    let s1 = String::from("hello");
    let s2 = s1;

    println!("{}, world!", s1); // compile time error as the ownership is transferred from s1 to s2
}