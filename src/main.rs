extern crate piston_window;

use piston_window::{PistonWindow, WindowSettings};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Unnamed N64 Emulator", [320, 240])
        .exit_on_esc(true)
        .build()
        .unwrap();

    while let Some(e) = window.next() {
        //TODO
    }
}
