use std::fs::File;
fn main() {
    let useless_file_result = File::open("./useless1.txt");

    let file_open = match useless_file_result {
        Ok(file) => { file }
        Err(error) => { panic!("Problem opening the file: {:?}", error) }
    };
}