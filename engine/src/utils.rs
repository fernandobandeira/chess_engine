pub const PLUS_INFINITY: i32 = 999999;
pub const MINUS_INFINITY: i32 = -PLUS_INFINITY;

pub fn send_output(output: &str) {
    println!("{}", output);
    log::debug!("Output: {}", output);
}