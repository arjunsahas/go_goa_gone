use std::fs::File;
fn main() {
    let useless_file_result = File::open("./useless1.txt");

    // handle using plain if else to check Result instead of match.
    if useless_file_result.is_ok() {
        println!("File already exists!");
    } else {
        println!("File does not exists!");
        println!("{} file created", "./useless1.txt");
        File::create("./useless1.txt");
    }
}