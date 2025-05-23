use std::io::{self, stdout, Write};
use std::process::Command;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    for arg in args {
        println!("{}", &arg);
    }

    for (key, value) in std::env::vars() {
        println!("{key}: {value}");
    }

    loop {
        print!("$ ");
        stdout().flush().unwrap_or_else(|error| {
            panic!("Could not write shell prompt: {error:?}");
        });

        let mut command = String::new();
        io::stdin()
        .read_line(&mut command)
        .expect("Quitting because we had a read error yippee");

        //Remove trailing newline
        let newline_pos = command.rfind("\n");

        match newline_pos {
            Some(position) => {command.remove(position);},
            None => (),
        }

        println!("Executing: {}", &command);

        if command == "exit" {
            return;
        }

        let command_vector: Vec<&str> = command.split(" ").collect();

        let command = match command_vector.get(0) {
            Some(val) => val,
            None => continue,
        };

        if command.len() < 1 {
            continue;
        }

        let mut command = Command::new(command);

        command.args(&command_vector[1..command_vector.len()]);

        let command = command.spawn();

        match command {
            Ok(mut child) => {
                stdout().flush().expect("Failed to write to screen");
                child.wait().expect("");
            },
            Err(error) => {
                println!("Failed to run command: {error:?}");
                stdout().flush().expect("Failed to write to screen");
                continue;
            }
        }
    }
}
