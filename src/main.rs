mod commands;
mod utils;
mod shell;

use std::sync::Arc;
use std::sync::atomic::AtomicBool;

fn main() {
    let is_terminated = Arc::new(AtomicBool::new(false));
    utils::setup_signal_handler(is_terminated.clone());
    
    shell::run_shell_loop(is_terminated);
}
