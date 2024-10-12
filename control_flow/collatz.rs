/// Determine the length of the collatz sequence beginning at `n`.
fn collatz_length(mut n: i32) -> u32 {
    let mut count = 1;
    while n != 1 {
        if n % 2 == 0 {
            n = n / 2;
            count = count + 1;
            println!("Value of n: {}", n);
            println!("count {}", count);
        } else {
            n = (3 * n) + 1;
            count = count + 1;
            println!("Value of n: {}", n);
            println!("count {}", count);
        }
    }
    count
}

#[test]
fn test_collatz_length() {
    assert_eq!(collatz_length(11), 15);
}

fn main() {
    println!("Length: {}", collatz_length(11));
}