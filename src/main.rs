mod file_handler;

fn main() {
    println!("Testing file");
    let filename: String = "Story/[C0,a].txt".to_string();
    let file = file_handler::open_file(filename);
    println!("Done");
    file_handler::print_file(file);
}
