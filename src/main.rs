use story_page::StoryPage;

mod file_handler;
mod story_page;

fn main() {
    let filename: String = String::from("Story/[C0,a].txt");
    let file_text = file_handler::open_file(filename);

    let story: StoryPage = StoryPage::new_story_page(file_text);
    println!("{}", story.text);
    println!("Status");
    println!("Current Code: {}, Option Codes: {:?}", story.current_code, story.option_codes);
}

/* 
// Type of Function, If I need it it's here
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/