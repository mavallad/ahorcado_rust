extern crate rodio;

use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

pub fn play_win() {
    play_file("sounds/win.ogg");
}

pub fn play_lose() {
    play_file("sounds/lose.ogg");
}

fn play_file(file_location: &str) {
    let device = rodio::default_output_device().unwrap();
    let file = File::open(file_location).unwrap();
    let mut beep = rodio::play_once(&device, BufReader::new(file)).unwrap();
    beep.set_volume(0.8);
    thread::sleep(Duration::from_millis(2000));
}
