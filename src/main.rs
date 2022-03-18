mod args;
mod sml;

use args::*;
use sml::*;

use notify::{Watcher, RecursiveMode, watcher};
use std::{fs, env};
use std::sync::mpsc::channel;
use std::time::{Duration, Instant};

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    parse_args(args[1..].to_vec());
    let (tx, rx) = channel();

    let mut watcher = watcher(tx, Duration::from_secs(1)).unwrap();

    match watcher.watch("./content", RecursiveMode::Recursive) {
        Ok(_) => {},
        Err(_) => {
            fs::create_dir_all("./content")?;
            watcher.watch("./content", RecursiveMode::Recursive).expect("failed to watch directory");
        }
    };

    let mut recompiler = Sml::new(None, None, None, None);

    loop {
        match rx.recv() {
            Ok(event) => {
                if !event_handler(&event) { continue; } 
                println!("pass: {:?}", event);
                let timer = Instant::now();
                recompiler.update();
                println!("completed in {:?}", timer.elapsed());
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }
}
