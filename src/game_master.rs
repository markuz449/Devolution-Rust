use std::io::*;
use termion::{event::Key, raw::RawTerminal};
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::cursor::{Hide, Show};
use termion::{color, style};

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
    pub enter_name_active: bool,
    pub is_girl: bool,
    pub gender_active: bool,
    pub continue_active: bool,
}

type Out = RawTerminal<Stdout>;

pub fn game_loop() {
    let stdin = stdin();
    let mut stdout: Out = stdout().into_raw_mode().unwrap();
    let mut help_active: bool = false;
    let mut character_creator_active: bool = false;

    // Creating character struct
    let name: String = String::from("");
    let enter_name_active: bool = true;
    let is_girl: bool = false;
    let gender_active: bool = false;
    let continue_active: bool = false;
    let mut character: Character = Character{name, enter_name_active, is_girl, gender_active, continue_active};

    // Getting the width of the terminal
    let terminal_width: usize = usize::from(termion::terminal_size().unwrap().0);

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
    
    stdout = write_title(&game_state, stdout);

    // Detecting keydown events
    for c in stdin.keys() {
        if character_creator_active{
            // Controls for the character creator
            match c.unwrap() {
                Key::Ctrl('c') => break,
                Key::Esc => break,
                Key::Left => {
                    if character.gender_active {
                        character.is_girl = false;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    }
                },
                Key::Right => {
                    if character.gender_active {
                        character.is_girl = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    }
                },
                Key::Up => {
                    if character.continue_active{
                        character.continue_active = false;
                        character.gender_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    } else if character.gender_active{
                        character.gender_active = false;
                        character.enter_name_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    }
                },
                Key::Down => {
                    if character.enter_name_active{
                        character.enter_name_active = false;
                        character.gender_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    } else if character.gender_active{
                        character.gender_active = false;
                        character.continue_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    }
                },
                Key::Char('\n') => {
                    if character.enter_name_active {
                        character.enter_name_active = false;
                        character.gender_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    } else if character.gender_active {
                        character.gender_active = false;
                        character.continue_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    }
                    else if character.name.len() != 0 && character.continue_active {
                        character_creator_active = false;
                        filename = String::from("Story/[C0].txt");
                        file_text = file_handler::open_text_file(filename, terminal_width);
                        story = StoryPage::initial_story_page(file_text, &character);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                Key::Backspace => {
                    if character.enter_name_active{
                        let name_length: usize = character.name.len();
                        if name_length > 0{
                            character.name.truncate(character.name.len() - 1);
                            stdout = write_character_creator(&character, &game_state, stdout);
                        }
                    }
                },
                Key::Char(c) => {
                    if character.enter_name_active{
                        let name_length: usize = character.name.len();
                        if name_length < 20 && c.is_alphabetic(){
                            character.name.push(c);
                            stdout = write_character_creator(&character, &game_state, stdout);
                        }
                    }
                },
                _ => {},
            }
        } else if help_active{
            // Controls for the help menu... which is anything
            match c.unwrap() {
                _ => ({
                    help_active = false;
                    stdout = write_story(&story, &game_state, stdout);
                }),
            }
        } else {
            // Main game controls
            match c.unwrap() {
                Key::Ctrl('c') => break,
                Key::Esc => {
                    write!(stdout, "{}", "\x1bc").unwrap();
                    break
                },
                Key::Char('h') => {
                    if !game_state.title_active{
                        help_active = true;
                        stdout = help(&game_state, stdout);
                    }
                },
                Key::Char('r') => {
                    game_state.title_active = true;
                    game_state.story_path.clear();
                    stdout = write_title(&game_state, stdout);
                },
                Key::Up => {
                    if !game_state.title_active && !story.game_over{
                        story = story.change_selected_option(-1);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                Key::Down => {
                    if !game_state.title_active && !story.game_over{
                        story = story.change_selected_option(1);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                Key::Left => {
                    game_state = re_read(&story, game_state, -1);
                    if game_state.re_read_mode {
                        story = open_previous_story(story, &game_state);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                Key::Right => {
                    if game_state.re_read_mode {
                        game_state = re_read(&story, game_state, 1);
                        story = open_previous_story(story, &game_state);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                Key::Char('\n') => {
                    if game_state.title_active{
                        game_state.title_active = false;
                        character_creator_active = true;
                        stdout = write_character_creator(&character, &game_state, stdout);
                    } else if story.game_over || game_state.re_read_mode{
                        continue;
                    } else{
                        game_state = update_story_path(&story, game_state);
                        story = submit_option(story, &game_state);
                        stdout = write_story(&story, &game_state, stdout);
                    }
                },
                _ => (),
            }
        }

        game_state.terminal_width = usize::from(termion::terminal_size().unwrap().0);
        stdout.flush().unwrap();
    }
    write!(stdout, "{}", Show).unwrap();
}

// Writes the title sequence and the planet to stdout
fn write_title(game_state: &GameState, mut stdout: Out) -> Out{
    write!(stdout, "{}{}", "\x1bc", style::Bold).unwrap();
    writeln!(stdout, "{}{}", color::Fg(color::Blue), format!("{:^1$}", game_state.planet, game_state.terminal_width)).unwrap();
    writeln!(stdout, "{}{}", color::Fg(color::Red), format!("{:^1$}", game_state.title, game_state.terminal_width)).unwrap();
    writeln!(stdout, "{}{}", color::Fg(color::LightWhite), style::Italic).unwrap();
    let start_message: String = format!("{} {}{}{} {}", "Press", color::Fg(color::Green), "Enter", color::Fg(color::LightWhite), "to Start");
    writeln!(stdout, "\r{}\r", format!("{:^1$}", start_message, game_state.terminal_width + 17)).unwrap();
    writeln!(stdout, "{}", Hide).unwrap();
    stdout
}

// Writes the Character creator
fn write_character_creator(character: &Character, game_state: &GameState, mut stdout: Out) -> Out {
    let width: usize = game_state.terminal_width;
    let no_name: bool = character.name.len() == 0;
    let title: String =         String::from("Create Your Character");
    let name_title: String =    String::from("What is you name?");
    let top_box: String =       String::from("╔════════════════════╗");
    let bottom_box: String =    String::from("╚════════════════════╝");
    let new_name: String =      format!("║{:<20}║", character.name);
    let name_error: String =    String::from("Please enter a name before continuing!");
    let gender_title: String =  String::from("What is your gender?");
    let arrow: String =         format!("{}{}{}{}", color::Fg(color::Blue), style::Blink, ">", style::NoBlink);
    let confirm_title: String = String::from("Continue with your character?");
    let mut confirm: String =   String::from("Continue");
    let boy: String;
    let girl: String;
    let gender_option: String;
    let name_box: String;
    
    if character.enter_name_active {
        name_box = format!("{}{} ", arrow, new_name);
        if character.is_girl{
            boy = format!("{}Boy", color::Fg(color::White));
            girl = format!("{}Girl", color::Fg(color::Green));
        } else{
            boy = format!("{}Boy", color::Fg(color::Green));
            girl = format!("{}Girl", color::Fg(color::White));
        }
        gender_option = format!("{}{:>10}{}", boy, " ", girl);
    } else if character.gender_active {
        name_box = format!("{}", new_name);
        if character.is_girl{
            boy = format!("{} Boy", color::Fg(color::White));
            girl = format!("{}{}Girl", color::Fg(color::Blue), arrow);
        } else{
            boy = format!("{}{}Boy", color::Fg(color::Blue), arrow);
            girl = format!("{} Girl", color::Fg(color::White));
        }
        gender_option = format!(" {}{:>9}{}", boy, " ", girl);
    } else {
        name_box = format!("{}", new_name);
        if character.is_girl{
            boy = format!("{}Boy", color::Fg(color::White));
            girl = format!("{}Girl", color::Fg(color::Green));
        } else{
            boy = format!("{}Boy", color::Fg(color::Green));
            girl = format!("{}Girl", color::Fg(color::White));
        }
        gender_option = format!("{}{:>10}{}", boy, " ", girl);
        confirm = format!("{}{}", arrow, confirm);
    }

    // Printing the character creator screen
    write!(stdout, "\r{}{}\r", "\x1bc", style::Bold).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Green), format!("{:^1$}", title, width)).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Yellow), format!("{:^1$}", name_title, width)).unwrap();
    if character.enter_name_active{
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Blue), format!("{:^1$}", top_box, width)).unwrap();
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Blue), format!("{:^1$}", name_box, width + 18)).unwrap();
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Blue), format!("{:^1$}", bottom_box, width)).unwrap();
    } else {
        writeln!(stdout, "\r{}{}\r", color::Fg(color::White), format!("{:^1$}", top_box, width)).unwrap();
        writeln!(stdout, "\r{}{}\r", color::Fg(color::White), format!("{:^1$}", name_box, width)).unwrap();
        writeln!(stdout, "\r{}{}\r", color::Fg(color::White), format!("{:^1$}", bottom_box, width)).unwrap();
    }
    if no_name{
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Red), format!("{:^1$}", name_error, width)).unwrap();
        writeln!(stdout, "").unwrap();
    } else{
        writeln!(stdout, "").unwrap();
        writeln!(stdout, "").unwrap();
    }
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Yellow), format!("{:^1$}", gender_title, width)).unwrap();
    writeln!(stdout, "").unwrap();
    if character.gender_active{
        writeln!(stdout, "\r{}\r", format!("{:^1$}", gender_option, width + (gender_option.len() / 2) + 6)).unwrap();
    } else {
        writeln!(stdout, "\r{}\r", format!("{:^1$}", gender_option, width + gender_option.len() / 2)).unwrap();
    }
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Yellow), format!("{:^1$}", confirm_title, width)).unwrap();
    writeln!(stdout, "").unwrap();
    if character.continue_active{
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Blue), format!("{:^1$}", confirm, width + 16)).unwrap();
    } else{
        writeln!(stdout, "\r{}{}\r", color::Fg(color::White), format!("{:^1$}", confirm, width)).unwrap();
    }
    writeln!(stdout, "{}", Hide).unwrap();
    stdout
}

// Writes the story page to stdout
fn write_story(story: &StoryPage, game_state: &GameState, mut stdout: Out) -> Out{
    write!(stdout, "{}{}", "\x1bc", style::Bold).unwrap();
    writeln!(stdout, "\r\n{}{}\r\n", color::Fg(color::Green), format!("{:^1$}", "Devolution", game_state.terminal_width)).unwrap();
    writeln!(stdout, "{}{}{}", color::Fg(color::White), style::Italic, story.text).unwrap();
    writeln!(stdout, "{}", style::NoItalic).unwrap();

    if !story.game_over && !game_state.re_read_mode {
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Yellow), format!("{:^1$}", "Choices", game_state.terminal_width)).unwrap();
        for i in 0..story.option_text.len(){
            if i == story.selection_num{
                write!(stdout, "\r{}{}\r", color::Fg(color::Blue), story.option_text[i]).unwrap();
            } else{
                write!(stdout, "\r{}{}\r", color::Fg(color::LightWhite), story.option_text[i]).unwrap();
            }
        }
    } else if game_state.re_read_mode {
        writeln!(stdout, "\r{}{}\r", color::Fg(color::Yellow), format!("{:^1$}", "Your Choice", game_state.terminal_width)).unwrap();
        let choice_num: usize = game_state.story_path[game_state.previous_story_num].choice_num;
        write!(stdout, "\r{}{}\r", color::Fg(color::Red), story.option_text[choice_num]).unwrap();
    }

    writeln!(stdout, "\r\n{}{}\r", color::Fg(color::Green), format!("{:^1$}", "To Quit, Press 'Esc'. For Help, Press 'h'", game_state.terminal_width)).unwrap();
    writeln!(stdout, "{}", Hide).unwrap();
    //print_story_status(&story, &game_state);
    stdout
}

// Writes the Help menu for the game
fn help(game_state: &GameState, mut stdout: Out) -> Out{
    let width: usize = game_state.terminal_width;
    let title: String =           String::from("Help Menu");
    let controls: String =        String::from("Controls:");
    let header: String =          String::from("Key           Action");
    let enter_control: String =   String::from("Enter         Continue the story with the selected option");
    let up_down_control: String = String::from("Up/Down       Select your choice");
    let re_read_control: String = String::from("Left/Right    Go back and forth through the story");
    let quit_control: String =    String::from("Esc           Exit out of the game");
    let help_control: String =    String::from("h             Open the help menu");
    let reset_control: String =   String::from("r             Resets the game");
    let exit: String =            String::from("Press Any Key to return to the game");

    // Printing the help screen
    write!(stdout, "\r{}{}\r", "\x1bc", style::Bold).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Green), format!("{:^1$}", title, width)).unwrap();
    writeln!(stdout, "{}{}", style::Italic, color::Fg(color::White)).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:^1$}", "Welcome to Devolution, a sci-fi adventure where you choose", width)).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:^1$}", "how you progresses!", width)).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "{}{}", style::NoItalic, color::Fg(color::Yellow)).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:^1$}", controls, width)).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}\r", color::Fg(color::Blue), format!("{:>1$}", header, (width / 3) + header.len())).unwrap();
    writeln!(stdout, "{}{}", style::Italic, color::Fg(color::White)).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", enter_control, (width / 3) + enter_control.len())).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", up_down_control, (width / 3) + up_down_control.len())).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", re_read_control, (width / 3) + re_read_control.len())).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", quit_control, (width / 3) + quit_control.len())).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", help_control, (width / 3) + help_control.len())).unwrap();
    writeln!(stdout, "\r{}\r", format!("{:>1$}", reset_control, (width / 3) + reset_control.len())).unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "").unwrap();
    writeln!(stdout, "\r{}{}{}\r", style::NoItalic, color::Fg(color::Green), format!("{:^1$}", exit, width)).unwrap();
    writeln!(stdout, "\r").unwrap();
    writeln!(stdout, "{}", Hide).unwrap();
    stdout
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
fn re_read(story: &StoryPage, mut game_state: GameState, direction: i8) -> GameState{
    // Ensures that there's a story path and the user isn't trying to go forward past the current story point
    if game_state.story_path.len() == 0 {
        return game_state
    }

    if !game_state.re_read_mode {
        game_state.re_read_mode = true;
        game_state.current_story_point = String::from(&story.current_file.clone());
        game_state.previous_story_num = game_state.story_path.len() - 1;
    } else {
        // Going back and forth through the story and makes sure that the user doesn't overflow
        if direction == -1 && game_state.previous_story_num > 0 {
            game_state.previous_story_num -= 1;
        } else if direction == 1 {
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
    println!("\n\rStatus of Re Read: {}\r", game_state.re_read_mode); 
    println!("\rStory Path Length: {}\r", game_state.story_path.len());
    println!("\rPrev Story Num: {}\r", game_state.previous_story_num);
    println!("\rCurrent Story Point: {}\r", game_state.current_story_point);
    println!("\r");
}

// Type of Function, If I need it it's here
#[allow(dead_code)]
fn print_type_of<T>(_: &T) {
    println!("\r{}\r", std::any::type_name::<T>())
}