use std::io::*;
use colored::*;
use termion::{event::Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::story_page::StoryPage;
use crate::file_handler;

#[allow(dead_code)]
struct GameState{
    story_path: Vec<StoryNode>,
    planet: String,
    title: String,
    title_active: bool,
    terminal_width: usize,
}
#[allow(dead_code)]
struct StoryNode{
    file_code: String,
    choice_num: usize,
}

pub fn game_loop() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut help_active: bool = false;

    // Getting the width of the terminal
    let terminal_size = termion::terminal_size().ok().expect("Failed to get terminal size");
    let terminal_width: usize = usize::from(terminal_size.0);

    // Opening title and sets game state
    let planet_file: String = String::from("Story/[PLANET].txt");
    let planet: String = file_handler::open_title_file(planet_file, terminal_width);
    let title_file: String = String::from("Story/[TITLE].txt");
    let title: String = file_handler::open_title_file(title_file, terminal_width);
    let title_active: bool = true;
    let story_path: Vec<StoryNode> = Vec::new();
    let mut game_state: GameState = GameState{story_path, planet, title, title_active, terminal_width};
    
    // Opening the first story file 
    let mut filename: String = String::from("Story/[C0].txt");
    let mut file_text: String = file_handler::open_text_file(filename, terminal_width);
    let mut story: StoryPage = StoryPage::new_story_page(file_text);

    print_story(&story, &game_state);

    // Detecting keydown events
    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c') => break,
            Key::Char('q') => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    break;
                }
            },
            Key::Esc => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    break;
                }
            },
            Key::Char('h') => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    help_active = true;
                    help(&game_state);
                }
            },
            Key::Char('r') => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    game_state.title_active = true;
                    filename = String::from("Story/[C0].txt");
                    file_text = file_handler::open_text_file(filename, terminal_width);
                    story = StoryPage::new_story_page(file_text);
                    print_story(&story, &game_state);
                }
            },
            Key::Up => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    story = change_option(story, -1, &game_state);
                }
            },
            Key::Down => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                } else{
                    story = change_option(story, 1, &game_state);
                }
            },
            Key::Char('\n') => {
                if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                }
                else if game_state.title_active{
                    game_state.title_active = false;
                    print_story(&story, &game_state);
                } else if story.game_over {
                    continue;
                } else{
                    story = submit_option(story, &game_state);
                    print_story(&story, &game_state);
                }
            },
            _ => (if help_active{
                    help_active = false;
                    print_story(&story, &game_state);
                }),
        }
        stdout.flush().unwrap();
    }
}

// Prints the Help menu for the game
fn help(game_state: &GameState){
    let width: usize = game_state.terminal_width;
    let title: ColoredString =            " Help Menu ".bold().green();
    let controls: ColoredString =         "Controls:".bold().yellow();
    let header: ColoredString =           "Key           Action".bold().blue();
    let enter_control: ColoredString =    "Enter         Continue the story with the selected option".italic();
    let up_down_control: ColoredString =  "Up/Down       Select your choice".italic();
    let quit_control: ColoredString =     "Esc/q         Exit out of the game".italic();
    let help_control: ColoredString =     "h             Open the help menu".italic();
    let reset_control: ColoredString =    "r             Resets the game".italic();
    let exit: ColoredString =             "  Press Any Key to return to the game  ".bold().green();

    // Printing the help screen
    print!("\r{}\r", "\x1bc");
    println!();
    println!("\r{}\r", format!("{:^1$}", title, width));
    println!();
    println!("\r{}\r", format!("{:^1$}", "Welcome to Devolution, a sci-fi adventure where you choose how".italic(), width));
    println!("\r{}\r", format!("{:^1$}", "the player progresses!".italic(), width));
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", controls, width));
    println!();
    println!("\r{}\r", format!("{:>1$}", header, (width / 5) + header.len()));
    println!("\r{}\r", format!("{:>1$}", enter_control, (width / 5) + enter_control.len()));
    println!("\r{}\r", format!("{:>1$}", up_down_control, (width / 5) + up_down_control.len()));
    println!("\r{}\r", format!("{:>1$}", quit_control, (width / 5) + quit_control.len()));
    println!("\r{}\r", format!("{:>1$}", help_control, (width / 5) + help_control.len()));
    println!("\r{}\r", format!("{:>1$}", reset_control, (width / 5) + reset_control.len()));
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", exit, width));
    print!("\r");
}

// Changes current option and reprints story
fn change_option(mut story: StoryPage, change: i8, game_state: &GameState) -> StoryPage{
    if !game_state.title_active && !story.game_over{
        story = story.change_selected_option(change);
        print_story(&story, &game_state);
    }
    story
}

// Submits option chosen by the user
fn submit_option(story: StoryPage, game_state: &GameState) -> StoryPage{
    let filename: String = format!("Story/{}.txt", story.option_codes[story.selection_num]);
    let file_text: String = file_handler::open_text_file(filename, game_state.terminal_width);
    let new_story: StoryPage = StoryPage::new_story_page(file_text);
    new_story
}

// Printing the story to terminal
fn print_story(story: &StoryPage, game_state: &GameState){
    if game_state.title_active {
        print!("{}", "\x1bc");
        println!("{}", format!("{:^1$}", game_state.planet.bold().blue(), game_state.terminal_width));
        println!("{}", format!("{:^1$}", game_state.title.bold().red(), game_state.terminal_width));
        let start_message: String = format!("{} {} {}", "Press".bold().italic(), "Enter".bold().italic().green(), "to Start".bold().italic());
        println!("\r{}\r", format!("{:^1$}", start_message, game_state.terminal_width + (start_message.len() / 2)));
    } else {
        print!("\x1bc");
        println!("\r\n{}\r\n", format!("{:^1$}", "Devolution".bold().green(), game_state.terminal_width));
        println!("{}", story.text.italic());

        if !story.game_over{
            println!("\r{}\r", format!("{:^1$}", "Choices".bold().italic().yellow(), game_state.terminal_width));
            for i in 0..story.option_text.len(){
                if i == story.selection_num{
                    print!("\r{}\r", story.option_text[i].blue().bold());
                } else{
                    print!("\r{}\r", story.option_text[i].bold());
                }
            }
        }
        println!("\r\n{}\r", format!("{:^1$}", "To Quit, Press 'q' or 'Esc'. For Help, Press 'h'".bold().italic().green(), game_state.terminal_width));
        //print_story_status(&story);
    }
}

/*** Supporting Print Functions (Remove when completed) ***/
#[allow(dead_code)]
fn print_story_status(story: &StoryPage){
    print!("\x1b[m");
    println!("\rStatus:\r");
    println!("\rCurrent File: {:?}, \n\rOption Codes: {:?}\r", story.current_file, story.option_codes);
    //println!("\rStory Text: {:?}\r", story.text);
}

// Type of Function, If I need it it's here
#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("\r{}\r", std::any::type_name::<T>())
}