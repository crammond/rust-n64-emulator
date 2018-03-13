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
    let contents = {
        let mut mut_contents: Vec<u8> = Vec::new();
        let _ = buf_reader.read_to_end(mut_contents.as_mut());
        mut_contents
    };
    // info via https://www.romhacking.net/forum/index.php?topic=20415.0
    println!("Initial PI settings: {:?}", &contents[0x0..0x4]);
    println!("Clock rate override: {:?}", &contents[0x4..0x8]);
    println!("Entry point: {:?}", &contents[0x8..0xC]);
    println!("Checksum: {:?}", &contents[0x10..0x18]);
    println!("Internal name: {}", String::from_utf8_lossy(&contents[0x20..0x33]));
    println!("Format: {}", char::from(contents[0x3B]));
    println!("GameID: {:?}", &contents[0x3C..0x3E]);
    println!("Country Code: {}", char::from(contents[0x3E]));
    println!("Version: {:?}", &contents[0x3F]);

    let mut window: PistonWindow = WindowSettings::new(format!("Unnamed N64 Emulator - {}", chosen_file), [320, 240])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(_) = window.next() {
        //TODO
    };
}
