mod file_handler;
mod text_manipulation;

fn main() {
    let filename: String = String::from("Story/[C0,a].txt");
    let file_text = file_handler::open_file(filename);
    print_file(&file_text);

    println!("Count: {}", text_manipulation::bracket_count(&file_text));
}

fn print_file(file: &String) {
    println!("{}", file);
}