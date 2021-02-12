use std::io::*;
use colored::*;
use termion::event::Key;
use termion::input::TermRead;
use termion::raw::IntoRawMode;

use crate::story_page::StoryPage;
use crate::file_handler;

struct GameState{
    story_path: Vec<StoryNode>,
    re_read_mode: bool,
    previous_story_num: usize,
    current_story_point: String,
    planet: String,
    title: String,
    title_active: bool,
    terminal_width: usize,
}
struct StoryNode{
    file_code: String,
    choice_num: usize,
}

pub struct Character{
    pub name: String,
    pub is_girl: bool,
}

pub fn game_loop() {
    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut help_active: bool = false;
    let mut character_creator_active: bool = false;

    // Creating character struct
    let name: String = String::from("");
    let is_girl: bool = false;
    let mut character: Character = Character{name, is_girl};

    // Getting the width of the terminal
    let terminal_size = termion::terminal_size().ok().expect("Failed to get terminal size");
    let terminal_width: usize = usize::from(terminal_size.0);

    // Declaring the Story variables 
    let mut filename: String;
    let mut file_text: String;
    let mut story: StoryPage = Default::default();

    // Opening title and sets game state
    let planet_file: String = String::from("Story/[PLANET].txt");
    let planet: String = file_handler::open_title_file(planet_file, terminal_width);
    let title_file: String = String::from("Story/[TITLE].txt");
    let title: String = file_handler::open_title_file(title_file, terminal_width);
    let title_active: bool = true;
    let story_path: Vec<StoryNode> = Vec::new();
    let re_read_mode: bool = false;
    let previous_story_num: usize = 0;
    let current_story_point: String = String::from("");
    let mut game_state: GameState = GameState{story_path, re_read_mode, previous_story_num, current_story_point, 
        planet, title, title_active, terminal_width};
    
    print_title(&game_state);

    // Detecting keydown events
    for c in stdin.keys() {
        if character_creator_active{
            // Controls for the character creator
            match c.unwrap() {
                Key::Ctrl('c') => break,
                Key::Esc => break,
                Key::Left => {
                    character.is_girl = false;
                    print_character_creator(&character, &game_state);
                },
                Key::Right => {
                    character.is_girl = true;
                    print_character_creator(&character, &game_state);
                },
                Key::Char('\n') => {
                    if character.name.len() == 0{
                        print_character_creator(&character, &game_state);
                    } else {
                        character_creator_active = false;
                        filename = String::from("Story/[C0].txt");
                        file_text = file_handler::open_text_file(filename, terminal_width);
                        story = StoryPage::initial_story_page(file_text, &character);
                        print_story(&story, &game_state);
                    }
                },
                Key::Backspace => {
                    let name_length: usize = character.name.len();
                    if name_length > 0{
                        character.name.truncate(character.name.len() - 1);
                        print_character_creator(&character, &game_state);
                    }
                },
                Key::Char(c) => {
                    let name_length: usize = character.name.len();
                    if name_length < 20{
                        character.name.push(c);
                        print_character_creator(&character, &game_state);
                    }
                },
                _ => {},
            }
        } else if help_active{
            // Controls for the help menu... which is anything
            match c.unwrap() {
                _ => ({
                    help_active = false;
                    print_story(&story, &game_state);
                }),
            }
        } else {
            // Main game controls
            match c.unwrap() {
                Key::Ctrl('c') => break,
                Key::Esc => {
                    print!("\x1bc");
                    break
                },
                Key::Char('h') => {
                    if !title_active{
                        help_active = true;
                        help(&game_state);
                    }
                },
                Key::Char('r') => {
                    game_state.title_active = true;
                    game_state.story_path.clear();
                    character_creator_active = true;
                    print_title(&game_state);
                },
                Key::Up => {
                    story = change_option(story, -1, &game_state);
                },
                Key::Down => {
                    story = change_option(story, 1, &game_state);
                },
                Key::Left => {
                    game_state = re_read(&story, game_state, true);
                    if game_state.re_read_mode {
                        story = open_previous_story(story, &game_state);
                        print_story(&story, &game_state);
                    }
                },
                Key::Right => {
                    game_state = re_read(&story, game_state, false);
                    story = open_previous_story(story, &game_state);
                    print_story(&story, &game_state);
                },
                Key::Char('\n') => {
                    if game_state.title_active{
                        game_state.title_active = false;
                        character_creator_active = true;
                        print_character_creator(&character, &game_state);
                    } else if story.game_over || game_state.re_read_mode{
                        continue;
                    } else{
                        game_state = update_story_path(&story, game_state);
                        story = submit_option(story, &game_state);
                        print_story(&story, &game_state);
                    }
                },
                _ => (),
            }
        }
        stdout.flush().unwrap();
    }
}

// Prints the title sequence and the planet
fn print_title(game_state: &GameState){
    print!("{}", "\x1bc");
    println!("{}", format!("{:^1$}", game_state.planet.bold().blue(), game_state.terminal_width));
    println!("{}", format!("{:^1$}", game_state.title.bold().red(), game_state.terminal_width));
    let start_message: String = format!("{} {} {}", "Press".bold().italic(), "Enter".bold().italic().green(), "to Start".bold().italic());
    println!("\r{}\r", format!("{:^1$}", start_message, game_state.terminal_width + (start_message.len() / 2)));
}

// Printing the story to terminal
fn print_story(story: &StoryPage, game_state: &GameState){
    print!("{}", "\x1bc");
    println!("\r\n{}\r\n", format!("{:^1$}", "Devolution".bold().green(), game_state.terminal_width));
    println!("{}", story.text.italic());

    if !story.game_over && !game_state.re_read_mode {
        println!("\r{}\r", format!("{:^1$}", "Choices".bold().italic().yellow(), game_state.terminal_width));
        for i in 0..story.option_text.len(){
            if i == story.selection_num{
                print!("\r{}\r", story.option_text[i].blue().bold());
            } else{
                print!("\r{}\r", story.option_text[i].bold());
            }
        }
    } else if game_state.re_read_mode {
        println!("\r{}\r", format!("{:^1$}", "Your Choice".bold().italic().yellow(), game_state.terminal_width));
        let choice_num: usize = game_state.story_path[game_state.previous_story_num].choice_num;
        print!("\r{}\r", story.option_text[choice_num].red().bold());
    }

    println!("\r\n{}\r", format!("{:^1$}", "To Quit, Press 'Esc'. For Help, Press 'h'".bold().italic().green(), game_state.terminal_width));
    //print_story_status(&story, &game_state);
}

// Prints the Help menu for the game
fn help(game_state: &GameState){
    let width: usize = game_state.terminal_width;
    let title: ColoredString =            " Help Menu ".bold().green();
    let controls: ColoredString =         "Controls:".bold().yellow();
    let header: ColoredString =           "Key           Action".bold().blue();
    let enter_control: ColoredString =    "Enter         Continue the story with the selected option".italic();
    let up_down_control: ColoredString =  "Up/Down       Select your choice".italic();
    let re_read_control: ColoredString =  "Left/Right    Go back and forth through the story".italic();
    let quit_control: ColoredString =     "Esc           Exit out of the game".italic();
    let help_control: ColoredString =     "h             Open the help menu".italic();
    let reset_control: ColoredString =    "r             Resets the game".italic();
    let exit: ColoredString =             "  Press Any Key to return to the game  ".bold().green();

    // Printing the help screen
    print!("\r{}\r", "\x1bc");
    println!();
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", title, width));
    println!();
    println!("\r{}\r", format!("{:^1$}", "Welcome to Devolution, a sci-fi adventure where you choose".italic(), width));
    println!("\r{}\r", format!("{:^1$}", "how you progresses!".italic(), width));
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", controls, width));
    println!();
    println!("\r{}\r", format!("{:>1$}", header, (width / 5) + header.len()));
    println!("\r{}\r", format!("{:>1$}", enter_control, (width / 5) + enter_control.len()));
    println!("\r{}\r", format!("{:>1$}", up_down_control, (width / 5) + up_down_control.len()));
    println!("\r{}\r", format!("{:>1$}", re_read_control, (width / 5) + re_read_control.len()));
    println!("\r{}\r", format!("{:>1$}", quit_control, (width / 5) + quit_control.len()));
    println!("\r{}\r", format!("{:>1$}", help_control, (width / 5) + help_control.len()));
    println!("\r{}\r", format!("{:>1$}", reset_control, (width / 5) + reset_control.len()));
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", exit, width));
    print!("\r");
}

// Prints the Character creator
fn print_character_creator(character: &Character, game_state: &GameState) {
    let width: usize = game_state.terminal_width;
    let no_name: bool = character.name.len() == 0;
    let name: ColoredString = character.name.bold().blue();
    let title: ColoredString =        "Create Your Character".bold().green();
    let name_title: ColoredString =   "What is you name?".bold().yellow();
    let top_box: ColoredString =      "╔════════════════════╗".bold();
    let bottom_box: ColoredString =   "╚════════════════════╝".bold();
    let name_box: String =    format!("              ║{:<20}║", name);
    let name_error: ColoredString =   "Please enter a name before continuing!".bold().red();
    let gender_title: ColoredString = "What is your gender?".bold().yellow();
    let mut boy: ColoredString =      "Boy".bold();
    let mut girl: ColoredString =     "Girl".bold();
    if character.is_girl{
        girl = girl.blue();
    } else{
        boy = boy.blue();
    }
    let gender_option: String = format!("{}        {}", boy, girl);
    let confirm: ColoredString = "Press 'Enter' to continue with your character".bold().green();

    // Printing the character creator screen
    print!("\r{}\r", "\x1bc");
    println!();
    println!();
    println!();
    println!("\r{}\r", format!("{:^1$}", title, width));
    println!();
    println!("\r{}\r", format!("{:^1$}", name_title, width));
    println!("\r{}\r", format!("{:^1$}", top_box, width));
    println!("\r{}\r", format!("{:^1$}", name_box.bold(), width));
    println!("\r{}\r", format!("{:^1$}", bottom_box, width));
    if no_name{
        println!("\r{}\r", format!("{:^1$}", name_error, width));
        println!();
    }
    println!();
    println!("\r{}\r", format!("{:^1$}", gender_title, width));
    println!();
    println!("\r{}\r", format!("{:^1$}", gender_option, width + (gender_option.len() / 2)));
    println!();
    println!("\r{}\r", format!("{:^1$}", confirm, width));
    println!();
}

// Changes current option and reprints story
fn change_option(mut story: StoryPage, change: i8, game_state: &GameState) -> StoryPage{
    if !game_state.title_active && !story.game_over{
        story = story.change_selected_option(change);
        print_story(&story, &game_state);
    }
    story
}

// Updates the users story path
fn update_story_path(story: &StoryPage, mut game_state: GameState) -> GameState{
    let file_code: String = String::from(&story.current_file.clone());
    let choice_num: usize = story.selection_num;
    let node: StoryNode = StoryNode{file_code, choice_num};
    game_state.story_path.push(node);
    game_state
}

// Submits option chosen by the user
fn submit_option(mut story: StoryPage, game_state: &GameState) -> StoryPage{
    let filename: String = format!("Story/{}.txt", story.option_codes[story.selection_num]);
    let file_text: String = file_handler::open_text_file(filename, game_state.terminal_width);
    story.text = file_text;
    story = StoryPage::new_story_page(story);
    story
}

// Handles when the user wants to re-read a previous part of the story
fn re_read(story: &StoryPage, mut game_state: GameState, go_back: bool) -> GameState{
    // Ensures that there's a story path and the user isn't trying to go forward past the current story point
    if game_state.story_path.len() == 0 || (!go_back && !game_state.re_read_mode){
        return game_state
    }

    if !game_state.re_read_mode {
        game_state.re_read_mode = true;
        game_state.current_story_point = String::from(&story.current_file.clone());
        game_state.previous_story_num = game_state.story_path.len() - 1;
    } else {
        // Going back and forth through the story and makes sure that the user doesn't overflow
        if go_back && game_state.previous_story_num > 0 {
            game_state.previous_story_num -= 1;
        } else if !go_back {
            game_state.previous_story_num += 1;
        }

        // When the user wants to go back to 
        if game_state.previous_story_num >= game_state.story_path.len(){
            game_state.re_read_mode = false;
        }
    }
    game_state
}

// Opens up a previous part of the story
fn open_previous_story(mut story: StoryPage, game_state: &GameState) -> StoryPage {
    let filename: String;
    if game_state.re_read_mode {
        filename = format!("Story/{}.txt", &game_state.story_path[game_state.previous_story_num].file_code);
    } else{
        filename = format!("Story/{}.txt", &game_state.current_story_point);
    }
    let file_text: String = file_handler::open_text_file(filename, game_state.terminal_width);
    story.text = file_text;
    story = StoryPage::new_story_page(story);
    story
}



/*** Supporting Print Functions (Remove when completed) ***/
#[allow(dead_code)]
fn print_story_status(story: &StoryPage, game_state: &GameState){
    print!("\x1b[m");
    println!("\rStatus:\r");
    println!("\rCurrent File: {:?}, \n\rOption Codes: {:?}\r", story.current_file, story.option_codes);
    print!("\rCurrent Story Path: ");
    for i in 0..game_state.story_path.len(){
        print!("[{}, {}], ", game_state.story_path[i].file_code, game_state.story_path[i].choice_num);
    }
    print!("\n\rStatus of Re Read: {}\n\rStory Path Length: {}\n\rPrev Story Num: {}\r",
    game_state.re_read_mode, game_state.story_path.len(), game_state.previous_story_num);
    println!("\r");
}

// Type of Function, If I need it it's here
#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("\r{}\r", std::any::type_name::<T>())
}