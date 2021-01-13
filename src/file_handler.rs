use std::fs;

pub fn open_file(filename: String) -> String {
    let f = fs::read_to_string(filename);
    let f = match f{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    f
}

pub fn print_file(file: String){
    println!("{}", file);
}