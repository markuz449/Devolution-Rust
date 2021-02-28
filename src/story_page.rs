use crate::game_master::Character;
use regex::Regex;

/// The Story page struct which holds all of the information about the current story file.
///
/// Struct Details:
/// * Text is the text for the current story page.
/// * Current_File is the current files code.
/// * Options_Codes is a list of the current choices file codes.
/// * Option_Text is a lis of the current choices text.
/// * Selection_Num is the current selected option.
/// * Game_Over tells if the current story page is a game over page.
/// * Name is the name of the users created character.
/// * Is_Girl is boolean value for is the character is a boy or girl, true for a girl and false for a boy.
#[derive(Debug, Default)]
pub struct StoryPage {
    pub text: String,
    pub current_file: String,
    pub option_codes: Vec<String>,
    pub option_text: Vec<String>,
    pub selection_num: usize,
    pub game_over: bool,
    pub name: String,
    pub is_girl: bool,
}

/// All of the story page fuctions.
/// These are used to clean the story into the struct from the raw file string.
impl StoryPage {

    /* Manipulation Functions */

    /// Generates the initial story page struct.
    /// It also sets the users character in the struct.
    pub fn initial_story_page(text: String, character: &Character) -> StoryPage {
        let current_file: String = String::from("");
        let option_codes: Vec<String> = Vec::new();
        let option_text: Vec<String> = Vec::new();
        let selection_num: usize = 0;
        let game_over: bool = false;
        let name: String = String::from(&character.name.clone());
        let is_girl: bool = character.is_girl;

        let mut story: StoryPage = StoryPage {
            text,
            current_file,
            option_codes,
            option_text,
            selection_num,
            game_over,
            name,
            is_girl,
        };

        story = Self::set_current_file(story);
        story = Self::replace_codes(story);
        story = Self::generate_choices(story);
        story
    }

    /// Updates the story page struct with it's updated raw file text.
    /// This function is used whenever the user chooses a different path in the story.
    /// This function preserves the users character options and fills in the story with their character details.
    pub fn new_story_page(mut story: StoryPage) -> StoryPage {
        story.option_codes.clear();
        story.option_text.clear();
        story.selection_num = 0;

        story = Self::set_current_file(story);
        story = Self::replace_codes(story);
        story = Self::generate_choices(story);
        story
    }

    /// Sets the curent file and removes the initial file code from the text.
    /// Each of the story files has their code at the very start of the file.
    /// This function removes it from the story text and sets it to be the current story point identifier.
    fn set_current_file(mut story: StoryPage) -> StoryPage {
        let start: usize = Self::find_indicies(&story.text, '[');
        let end: usize = Self::find_indicies(&story.text, ']');
        story.current_file = Self::get_slice(&story.text, start, end + 1);
        story = Self::remove_section(story, start, end + 1);
        story
    }

    /// Replaces the codes in text file with the users character details.
    /// Also sets the Game Over state if the game over code is detected.
    fn replace_codes(mut story: StoryPage) -> StoryPage {
        story.text = story.text.replace("[Name]", &story.name);
        if story.is_girl {
            story.text = story.text.replace("[Xe]", "she");
            story.text = story.text.replace("[Xer]", "her");
            story.text = story.text.replace("[Xis]", "her");
            story.text = story.text.replace("[Xers]", "hers");
            story.text = story.text.replace("[Xself]", "herself");
            story.text = story.text.replace("[Xther]", "sister");
            story.text = story.text.replace("[Xm]", "er");
            story.text = story.text.replace("[Xoy]", "girl");
        } else {
            story.text = story.text.replace("[Xe]", "he");
            story.text = story.text.replace("[Xer]", "him");
            story.text = story.text.replace("[Xis]", "his");
            story.text = story.text.replace("[Xers]", "his");
            story.text = story.text.replace("[Xself]", "himself");
            story.text = story.text.replace("[Xther]", "brother");
            story.text = story.text.replace("[Xm]", "em");
            story.text = story.text.replace("[Xoy]", "boy");
        }

        // Sets the game_over flag to true
        if story.text.contains("[Game Over]") {
            story.game_over = true;
            story.text = story.text.replace("[Game Over]", "");
            story.text = story.text.replace("[End]", "");
        }
        story
    }

    /// Generates the users choices for the current story file.
    /// Fills the option codes and option text arrays in the story struct.
    fn generate_choices(mut story: StoryPage) -> StoryPage {
        let bracket_num: usize = Self::bracket_count(&story.text);
        let mut start: usize;
        let mut end: usize;

        for _ in 0..bracket_num {
            // Saves the option code
            start = Self::find_indicies(&story.text, '[');
            end = Self::find_indicies(&story.text, ']');
            story.option_codes.push(Self::get_slice(&story.text, start, end + 1));
            story = Self::remove_section(story, start, end + 1);

            // Saves the option text
            // Checks if the end is reached
            if story.option_codes.contains(&String::from("[End]")) {
                break;
            }
            end = Self::find_indicies(&story.text, '[') - 1;
            start = Self::find_start_line(&story.text, start);
            end = Self::find_start_line(&story.text, end);
            story.option_text.push(Self::get_slice(&story.text, start, end + 1));
            story = Self::remove_section(story, start, end + 1);
        }
        story
    }

    /// Removes a part of the raw text between two given indicies.
    fn remove_section(mut story: StoryPage, start: usize, end: usize) -> StoryPage {
        story.text.replace_range(start..(end), "");
        story
    }

    /// Changes what the current selected choice.
    pub fn change_selected_option(mut self, change: i8) -> StoryPage {
        if change == 1 && self.selection_num < self.option_text.len() - 1 {
            self.selection_num += 1;
        } else if change == -1 && self.selection_num > 0 {
            self.selection_num -= 1;
        }
        self
    }


    /* Supporting Functions (Non-Manipulation) */

    /// Returns the number of start brackets that are found in a given piece of text.
    /// Start brackets are: '['.
    /// This function does this by using a regex query.
    fn bracket_count(text: &str) -> usize {
        let bracket = Regex::new(r"[\[]").unwrap();
        let count: usize = bracket.find_iter(&text).count();
        count
    }

    /// Returns the index number of the first discovered char in a piece of text.
    ///
    /// ## Panics
    ///
    /// Panics if the given text does not contain the given char.
    fn find_indicies(text: &str, find_char: char) -> usize {
        let find_index = text.find(find_char);
        let return_index: usize;
        match find_index {
            Some(found_index) => return_index = found_index,
            None => panic!("Problem finding char: {}", find_char),
        }
        return_index
    }

    /// Returns a subslice from a given string between two indices.
    ///
    /// ## Panics
    ///
    /// Panics if a subslice cannot be made from the two given indices.
    fn get_slice(text: &str, start: usize, end: usize) -> String {
        let mut clean_text: String = String::from(text);
        let slice_option = clean_text.get_mut(start..end);
        let slice: String;
        match slice_option {
            Some(x) => slice = String::from(x),
            None => panic!("There is no slice with given Start: {} and End {} indices", start, end),
        }
        slice
    }

    /// Sets the given index to the start of the line.
    /// It does this by checking each byte value until the New Line Feed or Carriage Return is found
    ///
    /// ## Example
    ///
    /// ```rust
    /// 10 == '\n' // New Line Feed 
    /// 13 == '\r' // Carriage Return
    /// ``` 
    fn find_start_line(text: &str, mut start: usize) -> usize {
        let byte_text: &[u8] = text.as_bytes();
        while byte_text[start] != 10 && start > 0 {
            start -= 1;
        }
        start
    }
}

#[cfg(test)]
mod tests {
    use crate::story_page::StoryPage;

    #[test]
    fn test_find_start_line() {
        let text = "\r\n    [C1]Hello World";
        let start: usize = StoryPage::find_indicies(text, '[');
        assert_eq!(StoryPage::find_start_line(text, start), 0);
    }

    #[test]
    fn test_fail_find_start_line() {
        let text = "\n    [C1]Hello World";
        let start: usize = StoryPage::find_indicies(text, '[');
        assert_eq!(StoryPage::find_start_line(text, start), 0);
    }
}
