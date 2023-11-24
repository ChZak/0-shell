use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::io::{self, Write};
use colored::*;
use crate::commands;
use crate::utils;

pub fn run_shell_loop(is_terminated: Arc<AtomicBool>) {
    loop {
        display_prompt();

        if is_terminated.load(Ordering::SeqCst) {
            break;
        }

        match read_user_input() {
            Some((command, args)) => {
                execute_command(command, args);
            }
            None => {
                if is_terminated.load(Ordering::SeqCst) {
                    break;
                }
            }
        }
    }
}

fn display_prompt() {
    let current_path = match std::env::current_dir() {
        Ok(path) => path.display().to_string(),
        Err(_) => String::from("Chemin inconnu"),
    };
    print!("{} $ ", current_path.blue());
    io::stdout().flush().unwrap();
}

fn read_user_input() -> Option<(String, std::str::SplitWhitespace)> {
    let mut input = String::new();
    if io::stdin().read_line(&mut input).is_ok() {
        let mut parts = input.trim().split_whitespace();
        let command = parts.next().unwrap_or("").to_string();
        Some((command, parts))
    } else {
        None
    }
}

fn execute_command(command: String, args: std::str::SplitWhitespace) {
    match command.as_str() {
        "exit" => std::process::exit(0),
        "cd" => commands::change_directory(args),
        "echo" => commands::echo(args),
        "pwd" => commands::pwd(),
        "ls" => commands::ls(args),
        "cat" => commands::cat(args),
        "cp" | "mv" | "rm" | "mkdir" => commands::run_system_command(&command, args),
        _ => utils::print_error(&format!("Commande '{}' non trouvée", command)),
    }
}