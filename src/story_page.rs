use regex::Regex;

pub struct StoryPage{
    pub text: String,
    pub current_file: String,
    pub option_codes: Vec<String>,
    pub option_text: Vec<String>,
    pub selection_num: usize,
    pub game_over: bool,
}

impl StoryPage{
    /*** Manipulation Functions ***/
    pub fn new_story_page(text: String) -> StoryPage{
        let current_file: String = String::from("");
        let option_codes: Vec<String> = Vec::new();
        let option_text: Vec<String> = Vec::new();
        let selection_num: usize = 0;
        let game_over: bool = false;

        let mut story: StoryPage = StoryPage{text, current_file, option_codes, option_text, selection_num, game_over};

        story = Self::set_current_file(story);
        story = Self::replace_codes(story);
        if !story.game_over{
            story = Self::generate_choices(story);
        }
        story
    }

    fn set_current_file(mut story: StoryPage) -> StoryPage {
        let start: usize = Self::find_indicies(&story.text, '[');
        let end: usize = Self::find_indicies(&story.text, ']');
        story.current_file = Self::get_slice(&story.text, start, end + 1);
        story = Self::remove_section(story, start, end);
        story
    }

    fn replace_codes(mut story: StoryPage) -> StoryPage {
        story.text = story.text.replace("[Name]", "Marcus");
        story.text = story.text.replace("[Xe]", "he");
        story.text = story.text.replace("[Xer]", "him");
        story.text = story.text.replace("[Xis]", "his");
        story.text = story.text.replace("[Xers]", "his");
        story.text = story.text.replace("[Xself]", "himself");
        story.text = story.text.replace("[Xther]", "brother");
        story.text = story.text.replace("[Xm]", "em");
        story.text = story.text.replace("[Xoy]", "boy");

        // Sets the game_over flag to true
        if story.text.contains("[Game Over]"){
            story.game_over = true;
            story.text = story.text.replace("[Game Over]", "");
            story.text = story.text.replace("[End]", "");
        }
        story
    }

    fn generate_choices(mut story: StoryPage) -> StoryPage {
        let bracket_num: usize = Self::bracket_count(&story.text);
        let mut start: usize;
        let mut end: usize;

        for _ in 0..bracket_num{
            // Saves the option code
            start = Self::find_indicies(&story.text, '[');
            end = Self::find_indicies(&story.text, ']');
            story.option_codes.push(Self::get_slice(&story.text, start, end + 1));
            story = Self::remove_section(story, start, end);
            
            // Saves the option text
            // Checks if the end is reached
            if story.option_codes.contains(&String::from("[End]")){
                break;
            }
            end = Self::find_indicies(&story.text, '[') - 1;
            story.option_text.push(Self::get_slice(&story.text, start, end));
            story = Self::remove_section(story, start, end);
        }
        story
    }

    fn remove_section(mut story: StoryPage, start: usize, end: usize) -> StoryPage {
        story.text.replace_range(start..(end + 1), "");
        story
    }

    pub fn change_selected_option(mut self, change: i8) -> StoryPage{
        if change == 1 && self.selection_num < self.option_text.len() - 1{
            self.selection_num += 1;
        }
        else if change == -1 && self.selection_num > 0{
            self.selection_num -= 1;
        }
        self
    }


    /*** Supporting Functions (Non-Manipulation) ***/
    fn bracket_count(text: &str) -> usize {
        let bracket = Regex::new(r"[\[]").unwrap();
        let count: usize = bracket.find_iter(&text).count();
        count
    }

    fn find_indicies(text: &str, find_char: char) -> usize {
        let find_index = text.find(find_char);
        let return_index: usize;
        match find_index {
            Some(found_index) => return_index = found_index,
            None => panic!("Problem finding char: {}", find_char),
        }
        return_index
    }

    fn get_slice(text: &str, start: usize, end: usize) -> String {
        let mut clean_text: String = String::from(text);
        let slice_option = clean_text.get_mut(start..end);
        let slice: String;
        match slice_option{
            Some(x) => slice = String::from(x),
            None => panic!("There is no slice with given Start: {} and End {} indexes", start, end),
        }
        slice
    }

}