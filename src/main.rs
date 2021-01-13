use std::fs;

fn main() {
    println!("Testing file");
    let file = open_file();
    print_file(file);
}

fn open_file() -> std::string::String {
    let file = fs::read_to_string("test.txt").expect("Error opening file");
    file
}

fn print_file(file: std::string::String){
    println!("{}", file);
}

