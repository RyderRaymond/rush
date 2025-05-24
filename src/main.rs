use std::env::set_current_dir;
use std::io::{self, stdout, Write};
use std::process::{exit, Command};
//use std::env;

fn main() -> ! {
    // let args: Vec<String> = env::args().collect();
    // for arg in args {
    //     println!("{}", &arg);
    // }

    // for (key, value) in std::env::vars() {
    //     println!("{key}: {value}");
    // }

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

        //println!("Executing: {}", &command);

        let command_vector: Vec<&str> = command.split(" ").collect();

        let command = match command_vector.get(0) {
            Some(val) => val,
            None => continue,
        };

        if command.len() < 1 {
            continue;
        }

        let argument_vector = match command_vector.len() > 1 {
            true => &command_vector[1..],
            false => &[]
        };

        if is_builtin_command(command, argument_vector) {
            continue;
        }

        let mut command = Command::new(command);

        command.args(argument_vector);

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

fn is_builtin_command(command: &str, argument_vector: &[&str]) -> bool {
    match command {
        "exit" => {
            builtin_exit(argument_vector);
            return true;
        }
        "cd" => {
            builtin_cd(argument_vector);
            return true;
        }
        _ => false
    }
}

fn usage_error(message: &str) {
    eprintln!("rush: {}", message);
}

fn builtin_exit(argument_vector: &[&str]) {
    if argument_vector.len() == 0 {
        exit(0);
    }

    let exit_val: i32 = match argument_vector[0].trim().parse::<i32>() {
        Ok(num) => num,
        Err(_) => {
            usage_error(&format!("exit {}: numeric argument required", argument_vector[0]));
            1
        }
    };

    exit(exit_val);
}

fn builtin_cd(argument_vector: &[&str]) {
    let directory = match argument_vector.get(0) {
        None => "/", //we'll have to implement $HOME later
        Some(dir) => dir
    };

    match set_current_dir(directory) {
        Ok(_) => (),
        Err(error) => eprintln!("{}", error)
    };
}
