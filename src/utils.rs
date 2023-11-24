use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use signal_hook::{iterator::Signals, consts::SIGINT};
use std::thread;
use colored::*;

/// Configure le gestionnaire pour le signal SIGINT (Ctrl+C).
pub fn setup_signal_handler(is_terminated: Arc<AtomicBool>) {
    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT]).unwrap();
        for sig in signals.forever() {
            if sig == SIGINT {
                is_terminated.store(true, Ordering::SeqCst);
                println!("\nInterruption Ctrl+C détéctée.");
            }
        }
    });
}

/// Affiche un message d'erreur avec une coloration rouge.
pub fn print_error(message: &str) {
    eprintln!("{}", message.red());
}