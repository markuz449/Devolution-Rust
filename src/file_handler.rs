use std::fs::File;
use std::io;
use std::process::{Command, Stdio};
use termion::input::TermRead;

/// Opens the title files and only pipes them through  the 'pr' command.
/// This fuction is only used for the title files because all it does is centering, no text formattting.
/// This is done so that the title and planet can preserve their shape.
///
/// ## Example
///
/// ```
/// pr -t -o 7 Story/"[TITLE].txt"
/// ```
///
/// ## Panics
///
/// Panics at every step if the it cannot complete.
pub fn open_title_file(filename: String, terminal_width: usize) -> String {
    let mut file = File::open(&filename).ok().expect("Failed to open file");
    let line_length: usize = file.read_line().unwrap().expect("Failed to get first line").len();
    let title_width: usize = terminal_width.checked_sub(line_length).unwrap_or_default();

    let mut pr = Command::new("pr")
        .args(&["-t", "-o", &(title_width / 2).to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .expect("failed to spawn process");

    io::copy(&mut file, pr.stdin.as_mut().unwrap()).ok().expect("Can't copy from FILE");
    let pr_output = pr.wait_with_output().unwrap();
    let mut story_file: String = String::from_utf8(pr_output.stdout).unwrap();
    story_file = story_file.replace('\n', "\r\n");
    
    story_file
}

/// Opens the file and pipes the text through the 'fmt' and 'pr' commands.
/// This will ensure that the text is nicely formatted and will best fit the terminal size.
///
/// ## Example
///
/// ```
/// fmt -s -w 70 Story/"[C0].txt" | pr -t -o 7
/// ```
///
/// ## Panics
///
/// Panics at every step if the it cannot complete.
pub fn open_text_file(filename: String, terminal_width: usize) -> String {
    let mut fmt = Command::new("fmt")
        .args(&["-s", "-w", &(terminal_width - (terminal_width / 5)).to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .expect("failed to spawn process");

    let mut pr = Command::new("pr")
        .args(&["-t", "-o", &(terminal_width / 10).to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok()
        .expect("failed to spawn process");

    let mut file = File::open(filename).ok().expect("Failed to open file");
    io::copy(&mut file, fmt.stdin.as_mut().unwrap()).ok().expect("Can't copy from FILE");
    let fmt_output = fmt.wait_with_output().unwrap();
    let fmt_text: String = String::from_utf8(fmt_output.stdout).unwrap();

    let mut pr_bytes = fmt_text.as_bytes();
    io::copy(&mut pr_bytes, pr.stdin.as_mut().unwrap()).ok().expect("Can't copy from Output");
    let pr_output = pr.wait_with_output().unwrap();
    let mut story_text: String = String::from_utf8(pr_output.stdout).unwrap();
    story_text = story_text.replace("\n", "\r\n");

    story_text
}
