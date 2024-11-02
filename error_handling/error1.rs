use std::fs::File;
fn main() {
    let useless_file = File::open("./useless.txt");

    let file_open = match useless_file {
        Ok(file) => {file},
        Err(error) => {panic!("Problem opening the file: {:?}", error)}
    };
}