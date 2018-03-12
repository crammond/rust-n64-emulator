extern crate piston_window;
extern crate nfd;

use nfd::Response;
use piston_window::{PistonWindow, WindowSettings};

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() {
    let result = nfd::open_file_dialog(None, None).unwrap();

    let chosen_file = match result {
        Response::Okay(file_path) => file_path,
        _ => "".to_string()
    };

    let file = File::open(&chosen_file).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents: Vec<u8> = Vec::new();
    let _ = buf_reader.read_to_end(contents.as_mut());
    // print the internal game name, info via https://www.romhacking.net/forum/index.php?topic=20415.0
    println!("Internal name of file {}\n {}", chosen_file, String::from_utf8_lossy(&contents[32..52]));

    let mut window: PistonWindow = WindowSettings::new(format!("Unnamed N64 Emulator - {}", chosen_file), [320, 240])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(_) = window.next() {
        //TODO
    };
}
