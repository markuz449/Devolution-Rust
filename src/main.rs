use std::io::*;
use colored::*;
use termion::{event::Key};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use story_page::StoryPage;

mod file_handler;
mod story_page;

struct GameState{
    story_path: Vec<StoryNode>,
    title: String,
    title_active: bool,
    terminal_width: usize,
}

struct StoryNode{
    file_code: String,
    choice_num: usize,
}

fn main() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut help_active: bool = false;

    // Getting the width of the terminal
    let terminal_size = termion::terminal_size().ok().expect("Failed to get terminal size");
    let terminal_width: usize = usize::from(terminal_size.0);

    // Opening title and sets game state
    let title_file: String = String::from("Story/[TITLE].txt");
    let title: String = file_handler::open_file(title_file);
    let title_active: bool = true;
    let story_path: Vec<StoryNode> = Vec::new();
    let mut game_state: GameState = GameState{story_path, title, title_active, terminal_width};
    
    // Opening the first story file
    let filename: String = String::from("Story/[C0].txt");
    let file_text: String = file_handler::open_text_file(filename, terminal_width);
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
        println!("{}{}", "\x1bc", game_state.title.bold());
        println!("\r 	{}{}{}\r", "Press ".bold().italic(), "Enter".bold().italic().green(), " to Start".bold().italic());
    } else {
        print!("\x1bc");
        println!("\r\n{}\r\n", format!("{:^1$}", "Devolution".bold().green(), game_state.terminal_width));
        println!("{}", story.text);

        if !story.game_over{
            println!("\r{}\r", format!("{:^1$}", "Choices".bold().italic().yellow(), game_state.terminal_width));
            for i in 0..story.option_text.len(){
                if i == story.selection_num{
                    println!("\r{}\r", story.option_text[i].blue().bold());
                } else{
                    println!("\r{}\r", story.option_text[i]);
                }
            }
        }
        println!("\r\n{}\r", format!("{:^1$}", "To Quit, Press 'q' or 'Esc'. For Help, Press 'h'".bold().italic().green(), game_state.terminal_width));
    }
}

/*** Supporting Print Functions (Remove when completed) ***/
fn print_story_status(story: &StoryPage){
    print!("\x1b[m");
    println!("\rStatus\r");
    println!("\rCurrent File: {:?}, Option Codes: {:?}\r", story.current_file, story.option_codes);
}

// Type of Function, If I need it it's here
fn print_type_of<T>(_: &T) {
    println!("\r{}\r", std::any::type_name::<T>())
}