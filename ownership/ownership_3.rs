fn main() {
    let s1 = String::from("hello");
    let len = length(&s1); // borrow
    println!("The length of '{}' is {}.", s1, len);

    let len1 = length1(&s1);
    println!("The length of '{}' is {}.", s1, len1);

    let s2 = s1;
    let len2 = length2(s2.clone());
    println!("The length of '{}' is {}.", s2, len2);
}

fn length(s: &String) -> usize {
    s.len()
}

fn length1(s: &String) -> usize {
    length(s) // borrow
}

fn length2(s: String) -> usize {
    s.len()
}