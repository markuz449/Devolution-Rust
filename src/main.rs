use std::io::*;
use colored::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use story_page::StoryPage;

mod file_handler;
mod story_page;

/** Try and get piping working with command: fmt -t Story/\[C0\].txt **/

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    // Opening the file
    let filename: String = String::from("Story/[C0].txt");
    let file_text = file_handler::open_file(filename);

    // Preparing the story
    let mut story: StoryPage = StoryPage::new_story_page(file_text);
    print_story(&story);
    
    // Detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('q') => break,
            Key::Char('h') => println!("Help?!\r"),
            Key::Up => {
                story = story.change_selected_option(-1);
                print_story(&story);
            },
            Key::Down => {
                story = story.change_selected_option(1);
                print_story(&story);
            },
            Key::Char('\n') => println!("Submit\r"),
            _ => (),
        }
        stdout.flush().unwrap();
    }
}

// Printing the story to terminal
fn print_story(story: &StoryPage){
    print!("\x1bc");
    println!("{}", story.text.bold());
    println!("\r{}", "*** Choices ***".bold().italic());
    for i in 0..story.option_text.len(){
        if i == story.selection_num{
            println!("\r{}", story.option_text[i].green().bold());
        } else{
            println!("\r{}", story.option_text[i].bold());
        }
    }
}

/*** Supporting Print Functions (Remove when completed) ***/
fn print_story_status(story: &StoryPage){
    print!("\x1b[m");
    println!("Status");
    println!("Story Path: {:?}, Option Codes: {:?}", story.story_path, story.option_codes);
}

// Type of Function, If I need it it's here
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}