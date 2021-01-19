use std::fs;

pub fn open_file(filename: String) -> String {
    println!("*** Opening file: {} ***", filename);
    let story_file = fs::read_to_string(filename);
    let f = match story_file{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    f
}

