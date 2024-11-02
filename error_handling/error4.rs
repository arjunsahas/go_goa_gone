use std::fs::File;
use std::io;

fn read_from_file() -> Result<String, io::Error> {
    let file_result = File::open("./useless2.txt");
    let file_match = match file_result {
        Ok(file) => {file}
        Err(error) => {return Err(error)}
    };
    file_match;
}

fn main() {
    let _result = read_from_file();
}