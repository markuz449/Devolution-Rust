use std::io::*;
use colored::*;
use termion::{event::Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use story_page::StoryPage;

mod file_handler;
mod story_page;

struct StoryPath{
    file_code: String,
    choice_num: usize,
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut title_active: bool = true;
    let mut story_path: Vec<StoryPath> = Vec::new();

    // Opening title
    let title_file: String = String::from("Story/[TITLE].txt");
    let title: String = file_handler::open_file(title_file);
    println!("{}", title.bold());
    println!("\r 	{}{}{}\r", "Press ".bold().italic(), "Enter".bold().italic().green(), " to Start".bold().italic());
    
    // Opening and setting the story file
    let filename: String = String::from("Story/[C0].txt");
    let file_text: String = file_handler::open_text_file(filename);
    let mut story: StoryPage = StoryPage::new_story_page(file_text);

    // Detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('q') => break,
            Key::Esc => break,
            Key::Char('h') => println!("\rHelp?!\r"),
            Key::Up => {
                story = change_option(story, -1, title_active);
            },
            Key::Down => {
                story = change_option(story, 1, title_active);
            },
            Key::Char('\n') => {
                if title_active{
                    title_active = false;
                    print_story(&story);
                } else {
                    submit_option();
                }
            },
            _ => (),
        }
        stdout.flush().unwrap();
    }
}

// Changes current option and reprints story
fn change_option(mut story: StoryPage, change: i8, title_active: bool) -> StoryPage{
    if !title_active {
        story = story.change_selected_option(change);
        print_story(&story);
    }
    story
}

// Submits option chosen by the user
fn submit_option(){
    println!("\rSubmit\r");
}

// Printing the story to terminal
fn print_story(story: &StoryPage){
    print!("\x1bc");
    println!("{}", story.text.bold());

    println!("\r{}\r", "*** Choices ***".bold().italic());
    for i in 0..story.option_text.len(){
        if i == story.selection_num{
            println!("\r{}\r", story.option_text[i].green().bold());
        } else{
            println!("\r{}\r", story.option_text[i].bold());
        }
    }
    println!("\r\n{}\r", "*** To Quit, press 'q' or 'Esc'. For Help, press 'h' ***".bold().italic());
}

/*** Supporting Print Functions (Remove when completed) ***/
fn print_story_status(story: &StoryPage){
    print!("\x1b[m");
    println!("Status");
    println!("Current File: {:?}, Option Codes: {:?}", story.current_file, story.option_codes);
}

// Type of Function, If I need it it's here
fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}