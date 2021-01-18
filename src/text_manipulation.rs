use regex::Regex;

pub fn get_bracket_strings(text: &str){
    let bracket_num: usize = bracket_count(text);
    let start: usize = find_indicies(text, '[');
    let end: usize = find_indicies(text, ']');

    println!("Bracket Num: {}, Start Index: {:?}, End Index: {:?}", bracket_num,  start, end);

    let slice: &str = &text[start..(end + 1)];
    println!("Slice: {}", slice);
}

fn bracket_count(text: &str) -> usize {
    let bracket = Regex::new(r"[\[]").unwrap();
    let count: usize = bracket.find_iter(&text).count();
    count
}

fn find_indicies(text: &str, find_char: char) -> usize{
    let find_index = text.find(find_char);
    let mut return_index = 0;
    let mut none_found: bool = false;
    match find_index {
        Some(found_index) => return_index = found_index,
        None => none_found = true,
    }
    if none_found{
        println!("Error finding given char: {}", find_char);
        println!("Exiting Program...");
        std::process::exit(0);
    }
    return_index
}