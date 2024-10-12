fn main() {
    println!("if condition");
    let mut number = 36;
    if number < 5 {
        println!("condition was true");
    } else if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    }

    println!("while loop");
    while number != 0 {
        println!("{}", number);
        number = number - 1;
    }

    println!("for loop");
    for number in 1..4 {
        println!("{}", number);
    }

    for number in (1..4).rev() {
        println!("{}", number);
    }

    for i in [1, 2, 3, 4, 5] {
        println!("{}", i);
    }

    let mut j = 0;
    loop {
        j += 1;
        println!("{}", j);
        if j == 10 {
            break;
        }
    }
}