use std::env;
use std::io::{self, Read, Write};
use std::process::{exit, Command};
use termion::raw::IntoRawMode;
//use std::env;

struct RushTerminal {
    program_name: String,
}

impl RushTerminal {
    fn repl_loop(&mut self) {
    //  let args: Vec<String> = env::args().collect();
        // for arg in args {
        //     println!("{}", &arg);
        // }

        // for (key, value) in std::env::vars() {
        //     println!("{key}: {value}");
        // }

        loop {
            io::stdout().flush();
            write!(io::stdout(), "$ ");
            io::stdout().flush().unwrap_or_else(|error| {
                panic!("{name}Could not write shell prompt: {error:?}", name=self.program_name);
            });

            // let mut command = String::new();
            // io::stdin()
            // .read_line(&mut command)
            // .expect("Quitting because we had a read error yippee");

            let mut command = self.get_command();

            //Remove trailing newline
            let newline_pos = command.rfind("\n");

            match newline_pos {
                Some(position) => {command.remove(position);},
                None => (),
            }

            //println!("Executing: {}", &command);

            let command_vector: Vec<&str> = command.split(" ").collect();

            let command_name = match command_vector.get(0) {
                Some(val) => val,
                None => continue,
            };

            if command_name.len() < 1 {
                continue;
            }

            let argument_vector = match command_vector.len() > 1 {
                true => &command_vector[1..],
                false => &[]
            };

            if self.is_builtin_command(command_name, argument_vector) {
                continue;
            }

            let mut command = Command::new(command_name);

            command.args(argument_vector);

            let command = command.spawn();

            match command {
                Ok(mut child) => {
                    io::stdout().flush().expect("Failed to write to screen");
                    child.wait().expect("");
                },
                Err(error) => {
                    println!("Failed to run command {command_name}: {}", error.to_string());
                    io::stdout().flush().expect("Failed to write to screen");
                }
            }
            io::stdout().flush();
            write!(io::stdout(), "\n\r");
            io::stdout().flush();
        }
    }

    fn get_command(&mut self) -> String {
        let mut stdout = io::stdout().lock().into_raw_mode().unwrap();
        let mut stdin = io::stdin();
        let mut command = String::new();

        loop {
            let mut buffer = [0; 1];

            stdin.read_exact(&mut buffer).unwrap();

            match buffer[0] {
                13 => {
                    command.push('\n');
                    write!(stdout, "\n\r");
                    stdout.flush();
                    break;
                },

                127 => {
                    print!("{}", command.len());
                    if command.len() > 0 {
                        command.pop();
                        stdout.write(&[8, 32]);
                        write!(stdout, "{}", termion::cursor::Left(1));
                        stdout.flush();
                        continue;
                    }
                }

                _ => ()
            }

            let curr_char = match str::from_utf8(&buffer) {
                Ok(ch) => ch,
                Err(_) => continue,
            };

            command.push_str(curr_char);
            stdout.write(&buffer).unwrap();
            stdout.flush().unwrap();
        }

        return command;
    }


    fn usage_error(&mut self, message: &str) {
        eprintln!("{name}: {m}", name=self.program_name, m=message);
    }

    fn builtin_exit(&mut self, argument_vector: &[&str]) {
        if argument_vector.len() == 0 {
            exit(0);
        }

        let exit_val: i32 = match argument_vector[0].trim().parse::<i32>() {
            Ok(num) => num,
            Err(_) => {
                self.usage_error(&format!("exit {}: numeric argument required", argument_vector[0]));
                1
            }
        };

        exit(exit_val);
    }

    fn builtin_cd(&mut self, argument_vector: &[&str]) {
        let directory = match argument_vector.get(0) {
            None => "/", //we'll have to implement $HOME later
            Some(dir) => dir
        };

        match env::set_current_dir(directory) {
            Ok(_) => (),
            Err(error) => eprintln!("{}", error)
        };
    }

    fn is_builtin_command(&mut self, command: &str, argument_vector: &[&str]) -> bool {
        match command {
            "exit" => {
                self.builtin_exit(argument_vector);
                return true;
            }
            "cd" => {
                self.builtin_cd(argument_vector);
                return true;
            }
            _ => false
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let prog_name = match args.get(0) {
        Some(name) => name,
        None => "rush" //default
    };

    let mut terminal = RushTerminal {
        program_name : String::from(prog_name),
    };

    terminal.repl_loop();
}

