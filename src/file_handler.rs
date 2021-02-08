use std::process::{ Command, Stdio };
use std::fs::*;
use std::io;

// Opens a file without piping
pub fn open_file(filename: String) -> String {
    let option_file = read_to_string(filename);
    let mut story_file = match option_file{
        Ok(file) => file,
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
    story_file = story_file.replace('\n', "\r\n");
    story_file
}

// Opens the file and pipes the text through fmt and pr before returning
// Running this command: fmt -s -w 70 Story/"[C0].txt" | pr -t -o 7
pub fn open_text_file(filename: String, terminal_width: usize) -> String {
    let mut fmt = Command::new("fmt")
        .args(&["-s", "-w", &(terminal_width - (terminal_width / 5)).to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok().expect("failed to spawn process");

    let mut pr = Command::new("pr")
        .args(&["-t", "-o", &(terminal_width / 10).to_string()])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok().expect("failed to spawn process");

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
