use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::{Command, Stdio};

fn green(text: &str) -> String {
    format!("\x1b[32m{}\x1b[0m", text)
}

fn main() {
    // Stock le répertoire courant(racine) y retourner avec la commande cd sans argument
    let project_base = env::current_dir().unwrap(); 

    loop {
        let current_path = env::current_dir().unwrap().display().to_string();
        print!("{} $ ", green(&current_path));
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        match input {
            "exit" => break,
            command if command.starts_with("cd") => {
                let parts: Vec<&str> = command.split_whitespace().collect();
                let new_dir = if parts.len() > 1 {
                    PathBuf::from(parts[1])
                } else {
                    project_base.clone()
                };

                if let Err(e) = env::set_current_dir(new_dir) {
                    eprintln!("cd: {}", e);
                }
            },
            command => {
                let mut child = Command::new("zsh")
                    .arg("-c")
                    .arg(command)
                    .stdout(Stdio::inherit())
                    .stderr(Stdio::inherit())
                    .spawn()
                    .expect("failed to execute process");

                let _ = child.wait().expect("failed to wait on child");
            }
        }
    }
}
