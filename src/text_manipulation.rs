use regex::Regex;

pub struct StoryPage{
    pub text: String,
    pub current_code: String,
    pub previous_code: String,
    pub option_codes: Vec<String>
}

impl StoryPage{
    /*** Manipulation Functions ***/
    pub fn new_story_page(text: String) -> StoryPage{
        let current_code: String = String::from(""); 
        let previous_code: String = String::from("");
        let option_codes: Vec<String> = Vec::new();

        let mut story: StoryPage = StoryPage{text, current_code, previous_code, option_codes};

        let bracket_num: usize = Self::bracket_count(&story.text);
        if bracket_num < 2{
            panic!("Error in files text as there is less than two bracketed codes!!");
        }

        story = Self::get_current_code(story);
        story = Self::get_choices(story, bracket_num -1);
        story
    }

    fn get_current_code(mut story: StoryPage) -> StoryPage {
        let start: usize = Self::find_indicies(&story.text, '[');
        let end: usize = Self::find_indicies(&story.text, ']');
        story.current_code = Self::get_slice(&story.text, start, end);
        story = Self::remove_brackets(story, start, end);
        story
    }

    fn get_choices(mut story: StoryPage, bracket_num: usize) -> StoryPage {
        let mut start: usize;
        let mut end: usize;

        for _ in 0..bracket_num{
            start = Self::find_indicies(&story.text, '[');
            end = Self::find_indicies(&story.text, ']');
            story.option_codes.push(Self::get_slice(&story.text, start, end));
            story = Self::remove_brackets(story, start, end);
        }
        story
    }

    fn remove_brackets(mut story: StoryPage, start: usize, end: usize) -> StoryPage {
        story.text.replace_range(start..(end + 1), "");
        story
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
        let slice_option = clean_text.get_mut(start..(end + 1));
        let slice: String;
        match slice_option{
            Some(x) => slice = String::from(x),
            None => panic!("There is no slice with given Start: {} and End {} indexes", start, end),
        }
        slice
    }

}
