use renki_core::RenkiCore;
use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = args[1].clone();
    let paths = fs::read_dir(&path).expect("Failed to scan directory");
    let mut filenames = Vec::new();
    for path in paths {
        if path.is_ok() {
            filenames.push(path.unwrap().path().to_str().unwrap().to_string());
        }
    }
    println!("Found {} images", filenames.len());
    let scenario_len = args[2].parse::<usize>().unwrap();
    RenkiCore::render_images(&filenames, scenario_len);
}
