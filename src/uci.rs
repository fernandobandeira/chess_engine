use std::sync::{Arc, Mutex, mpsc};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;

use crate::engine::{Engine};

pub fn listen(input_receiver: Receiver<String>) {
    let (stop_sender, stop_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let engine: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::new(stop_receiver)));

    loop {
        let input = input_receiver.recv().unwrap();
        let engine = engine.clone();

        match input.trim() {
            "uci" => init(),
            "isready" => isready(),
            "ucinewgame" => new_game(),
            "stop" => {
                stop_sender.send(true).unwrap()
            },
            "quit" => quit(),
            _ => {
                thread::spawn(move || {
                    let input = input.as_str();

                    if input.starts_with("position") {
                        engine.lock().unwrap().position(input);
                    }
        
                    if input.starts_with("go") {
                        engine.lock().unwrap().calculate_best_move();
                    }
                });
            }
        }
    }
}

// Display the engine information
// TODO add configuration options
fn init() {
    println!("id name RustyFish");
    println!("id author Fernando Bandeira");
    println!("uciok");
}

// TODO: Implement
fn new_game() {
    // Should prepare the engine for a new game
}

fn isready() {
    println!("readyok")
}

fn quit() {
    std::process::exit(0);
}
