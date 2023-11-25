use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use signal_hook::{iterator::Signals, consts::SIGINT};
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use colored::*;


fn main() {
    let is_terminated = Arc::new(AtomicBool::new(false));
    let is_terminated_clone = Arc::clone(&is_terminated);

    thread::spawn(move || {
        let mut signals = Signals::new(&[SIGINT]).unwrap();
        for sig in signals.forever() {
            if sig == SIGINT {
                is_terminated_clone.store(true, Ordering::SeqCst);
                println!("\nInterruption Ctrl+C détéctée. Appuez sur 'exit' pour quitter.");
            }
        }
    });
    loop {
        let current_path = env::current_dir().unwrap().display().to_string();
        print!("{} $ ", current_path);
        io::stdout().flush().unwrap();

        if is_terminated.load(Ordering::SeqCst) {
            break;
        }

        let mut input = String::new();
        if let Err(_) = io::stdin().read_line(&mut input) {
            if is_terminated.load(Ordering::SeqCst) {
                break;
            }
            continue;
        }

        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("");

        match command {
            "exit" => break,
            "cd" => {
                let new_dir = parts.next().unwrap_or("/");
                if let Err(e) = env::set_current_dir(Path::new(new_dir)) {
                    eprintln!("cd: {}", e);
                }
            },
            "echo" => {
                let echo_output = parts.collect::<Vec<&str>>().join(" ");
                println!("{}", echo_output);
            },
            "pwd" => {
                let output = Command::new("pwd").output().unwrap();
                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            "ls" => {
                let args = parts.collect::<Vec<&str>>();
                let mut command = Command::new("ls");
                command.args(args);
                let output = command.output().unwrap();

                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            "cat" => {
                let args = parts.collect::<Vec<&str>>();
                let mut command = Command::new("cat");
                command.args(args);
                let output = command.output().unwrap();

                println!("{}", String::from_utf8_lossy(&output.stdout));
            },
            "cp" | "mv" | "rm" | "mkdir" => {
                let args = parts.collect::<Vec<&str>>();
                let output = Command::new(command)
                    .args(args)
                    .output()
                    .unwrap();

                if !output.stderr.is_empty() {
                    eprintln!("{}", String::from_utf8_lossy(&output.stderr));
                }
            }
            _ => println!("Commande '{}' non trouvée", command.red()),
        }
    }
}
