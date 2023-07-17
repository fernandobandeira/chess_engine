use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

pub mod engine;
pub mod uci;
pub mod score;
pub mod utils;

fn main() {
    // Start a channel to communicate between the UCI thread and the main thread
    let (uci_sender, uci_receiver): (Sender<String>, Receiver<String>) = mpsc::channel();

    // Spawn a thread to listen for UCI commands
    thread::spawn(move || {
        uci::listen(uci_receiver);
    });

    // Listen for input from the user and send it to the UCI thread
    loop {
        let mut input: String = String::new();
        match std::io::stdin().read_line(&mut input) {
            Ok(_) => {
                let result = uci_sender.send(input);
                if result.is_err() {
                    log::error!("Error sending input: {}", result.unwrap_err());
                    std::process::exit(1);
                }
            }
            Err(error) => {
                log::error!("Error reading input: {}", error);
                std::process::exit(1);
            }
        }
    }
}
