use regex::Regex;

pub fn bracket_count(text: &String) -> usize {
    let bracket = Regex::new(r"[\[]").unwrap();
    let count: usize = bracket.find_iter(&text).count();
    count
}