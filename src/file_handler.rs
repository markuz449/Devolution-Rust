use std::process::{ Command, Stdio };
use std::fs::*;
use std::io;

pub fn open_file(filename: String) -> String {
    //println!("*** Opening file: {} ***", filename);
    let mut child = Command::new("fmt")
        .args(&["-t"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .ok().expect("failed to spawn process");


    let mut file = File::open(filename).ok().expect("Failed to open file");
    io::copy(&mut file, child.stdin.as_mut().unwrap()).ok().expect("Can't copy from FILE");
    let output = child.wait_with_output().unwrap();
    let mut story_text:String = String::from_utf8(output.stdout).unwrap();
    story_text = story_text.replace("\n", "\r\n");

    story_text
}