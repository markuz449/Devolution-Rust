use regex::Regex;

pub fn get_bracket_strings(mut text: String) -> Vec<String>{
    let bracket_num: usize = bracket_count(&text);
    let mut bracket_strings: Vec<String> = Vec::new();
    let mut start: usize;
    let mut end: usize;

    for _ in 0..bracket_num{
        start = find_indicies(&text, '[');
        end = find_indicies(&text, ']');

        println!("Bracket Num: {}, Start Index: {:?}, End Index: {:?}", bracket_num,  start, end);

        bracket_strings.push(get_slice(&text, start, end));
        println!("Slices: {:?}", bracket_strings);

        println!("Removing Brackets...");
        text = remove_brackets(text, start, end);
    }

    println!("Clean Text: {}", text);
    bracket_strings
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

fn get_slice(text: &str, start: usize, end: usize) -> String{
    let mut clean_text: String = String::from(text);
    let slice_option = clean_text.get_mut(start..(end + 1));
    let mut slice: String = String::from("");
    match slice_option{
        Some(x) => slice = x.to_string(),
        None => println!("There is no slice"),
    }
    if slice.eq(""){
        println!("Exiting Program...");
        std::process::exit(0);
    }
    slice
}

fn remove_brackets(mut text: String, start: usize, end: usize) -> String{
    text.replace_range(start..(end + 1), "");
    text
}