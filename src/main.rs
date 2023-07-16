pub mod uci;
pub mod logger;
pub mod strategies;

fn main() {
    let _ = logger::configure_logger();
    uci::main();
}
