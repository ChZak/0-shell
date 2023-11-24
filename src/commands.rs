use std::process::Command;
use std::env;
use std::path::Path;
use colored::*;

pub fn change_directory(mut parts: std::str::SplitWhitespace) {
    let new_dir = parts.next().unwrap_or("/");
    if let Err(e) = env::set_current_dir(Path::new(new_dir)) {
        eprintln!("cd: {}", e.to_string().red());
    }
}


pub fn echo(parts: std::str::SplitWhitespace) {
    let echo_output = parts.collect::<Vec<&str>>().join(" ");
    println!("{}", echo_output);
}

pub fn pwd() {
    let output = Command::new("pwd").output().unwrap();
    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn ls(parts: std::str::SplitWhitespace) {
    let args = parts.collect::<Vec<&str>>();
    let output = Command::new("ls")
        .args(args)
        .output()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn cat(parts: std::str::SplitWhitespace) {
    let args = parts.collect::<Vec<&str>>();
    let output = Command::new("cat")
        .args(args)
        .output()
        .unwrap();

    println!("{}", String::from_utf8_lossy(&output.stdout));
}

pub fn run_system_command(command: &str, parts: std::str::SplitWhitespace) {
    let args = parts.collect::<Vec<&str>>();
    let output = Command::new(command)
        .args(args)
        .output()
        .unwrap();

    if !output.stderr.is_empty() {
        eprintln!("{}", String::from_utf8_lossy(&output.stderr).red());
    } else {
        println!("{}", String::from_utf8_lossy(&output.stdout));
    }
}
