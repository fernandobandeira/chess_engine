use std::sync::mpsc::{Receiver, Sender};
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

use crate::engine::Engine;
use crate::utils;

pub fn listen(input_receiver: Receiver<String>) {
    let (stop_sender, stop_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();

    let engine: Arc<Mutex<Engine>> = Arc::new(Mutex::new(Engine::new(stop_receiver)));

    loop {
        let input = input_receiver.recv().unwrap();
        log::debug!("Input: {}", input.trim());
        let engine = engine.clone();
        let stop_sender = stop_sender.clone();

        match input.trim() {
            "uci" => init(),
            "isready" => isready(),
            "stop" => stop_sender.send(true).unwrap(),
            "quit" => quit(),
            _ => {
                let input = input.as_str();

                if input == "ucinewgame" {
                    engine.lock().unwrap().new_game();
                    continue;
                }

                // Example position startpos moves a2a3 b7b6
                if input.starts_with("position") {
                    engine.lock().unwrap().position(input);
                    continue;
                }

                // Example go movetime 3000
                if input.starts_with("go") {
                    thread::spawn(move || {
                        let (cancel_sender, cancel_receiver): (Sender<bool>, Receiver<bool>) = mpsc::channel();
                        thread::spawn(move || {
                            thread::sleep(std::time::Duration::from_secs(6));
                            if cancel_receiver.try_recv().is_ok() {
                                return;   
                            }

                            stop_sender.send(true).unwrap();
                        });

                        engine.lock().unwrap().calculate_best_move();
                        let _ = cancel_sender.send(true);
                    });
                }
            }
        }
    }
}

// Display the engine information
// TODO add configuration options
fn init() {
    utils::send_output("id name RustyFish");
    utils::send_output("id author Fernando Bandeira");
    utils::send_output("uciok");
}

fn isready() {
    utils::send_output("readyok")
}

fn quit() {
    std::process::exit(0);
}
