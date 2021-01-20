use std::fs;

pub fn open_file(filename: String) -> String {
    //println!("*** Opening file: {} ***", filename);
    let option_file = fs::read_to_string(filename);
    let story_file = match option_file{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    story_file
}

