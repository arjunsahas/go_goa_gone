fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);
    // as s1 is cloned, the ownership is not transferred but is duplicated.

    let i = 10;
    let j = i;
    println!("i = {}, j = {}", i, j);
    // This does not apply to simple types like integer which is stored in stack. The size of the variable is known which makes it easier for rust to handle in stack.
}