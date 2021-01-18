mod file_handler;
mod text_manipulation;

fn main() {
    let filename: String = String::from("Story/[C0,a].txt");
    let file_text = file_handler::open_file(filename);
    print_file(&file_text);

    println!("Bracket Strings: {:?}", text_manipulation::get_bracket_strings(&file_text));
}

fn print_file(file: &String) {
    println!("{}", file);
}

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}