use std::io::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use story_page::StoryPage;

mod file_handler;
mod story_page;

fn main() {
    // Opening the file
    let filename: String = String::from("Story/[C0].txt");
    let file_text = file_handler::open_file(filename);

    // Preparing the story
    let mut story: StoryPage = StoryPage::new_story_page(file_text);
    story.print_story_text();
    story.print_story_choices();

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    
    // Detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('h') => println!("Help?!\r"),
            Key::Up => story = story.change_selected_option(-1),
            Key::Down => story = story.change_selected_option(1),
            Key::Char('\n') => println!("Submit\r"),
            _ => (),
        }
        stdout.flush().unwrap();
    }
}

/* 
// Type of Function, If I need it it's here
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}*/