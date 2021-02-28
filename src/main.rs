//! Welcome to Devolution
//!
//! Devolution is a Sci-fi choose your own adventure game set in a dystopian future.

/// Holds the functions for opening and managing the story files.
mod file_handler;
/// Manages all of the game logic and prints the story to the terminal.
mod game_master;
/// Cleans the raw file text and holds all the current story point information in a struct.
mod story_page;

/// The main method for Devolution which starts the game loop.
fn main() {
    game_master::game_loop();
}
