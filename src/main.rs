use std::env;
use std::io::{self, Write};
use std::process::{exit, Command};
use termion::cursor::DetectCursorPos;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::event::Key;
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

            let mut command = match self.get_command() {
                Ok(string) => string,
                Err(error) => {
                    println!("Encountered error while getting command: {}", error);
                    continue;
                }
            };

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

    fn get_command(&mut self) -> Result<String, io::Error> {
        let mut stdout = io::stdout().lock().into_raw_mode().unwrap();
        let stdin = io::stdin();
        let mut command = String::new();
        let mut position_in_command: u16 = 0;
        let original_cursor_position_on_screen = stdout.cursor_pos()?;

        for k in stdin.keys() {
            match k.as_ref().unwrap() {
                Key::Char('\n') => {
                    command.push('\n');
                    write!(stdout, "\n\r")?;
                    stdout.flush()?;
                    break;
                },

                Key::Char(c) => {
                    command.insert(position_in_command.into(), *c);
                    position_in_command += 1;
                },

                Key::Left => {
                    if position_in_command > 0 {
                        write!(stdout, "{}", termion::cursor::Left(1))?;
                        stdout.flush()?;
                        position_in_command -= 1;
                    }
                    continue;
                },

                Key::Right => {
                    if usize::from(position_in_command) < command.len() {
                        write!(stdout, "{}", termion::cursor::Right(1))?;
                        stdout.flush()?;
                        position_in_command += 1;
                    }
                    continue;
                },

                //Going to need to make this actually remove at the right position
                Key::Backspace => {
                    if position_in_command > 0 && usize::from(position_in_command) <= command.len() {
                        command.remove(usize::from(position_in_command) - 1);
                        position_in_command -= 1;
                    }
                    else {
                        continue;
                    }
                },

                Key::Ctrl('c') => {
                    write!(stdout, "\n\r")?;
                    stdout.flush()?;
                    return Ok(String::from(""));
                },

                _ => {
                    println!("{:?}", k);
                },
            }

            stdout.flush()?;
            write!(stdout, "{}{}{}",
                termion::cursor::Goto(original_cursor_position_on_screen.0, original_cursor_position_on_screen.1),
                termion::clear::UntilNewline,
                command)?;

            stdout.flush()?;

            write!(stdout, "{}", termion::cursor::Goto(original_cursor_position_on_screen.0 + position_in_command, original_cursor_position_on_screen.1))?;
            stdout.flush()?;
        }


        return Ok(command);
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

