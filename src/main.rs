extern crate piston_window;

use piston_window::{EventLoop, Events, EventSettings, PistonWindow, WindowSettings};

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Unnamed N64 Emulator", [320, 240])
        .exit_on_esc(true)
        .build()
        .unwrap();

    let mut events = Events::new(EventSettings::new());
    while let Some(e) = events.next(&mut window) {
        //TODO
    }
}
