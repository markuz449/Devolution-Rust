use std::fs;

pub fn open_file(filename: String) -> String {
    println!("*** Opening file: {} ***", filename);
    let f = fs::read_to_string(filename);
    let f = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    f
}

