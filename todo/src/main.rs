use ::std::io::{Error, Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
}

#[derive(Debug)]
enum TerminalError {
    Stdout(Error),
    Stdin(Error),
}

#[derive(Debug, Clone)]
struct Todo {
    message: String,
}

impl Todo {
    fn new(message: String) -> Self {
        Todo { message }
    }
}

impl Terminal {
    fn new() -> Self {
        Terminal {
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
        }
    }

    fn ask_new_todo(&mut self) -> Result<Option<Todo>, TerminalError> {
        println!("Write your new todo:");
        self.input()
        .map(|user_input| {
            if user_input.is_empty() {
                None
            } else {
                Some(Todo::new(user_input))
            }
        })

    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "[ ] - {}", todo.message)
            .map_err(|error| TerminalError::Stdout(error))
    }

    fn user_intention(&mut self) -> bool {
        println!("Do you want to input a new todo? (y/n)");
        let user_input = self.input();
        if let Ok(input) = user_input {
            input == "y"
        } else {
            false
        }
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(|error| TerminalError::Stdin(error))
            .map(|_| buf.trim().to_string())
    }

    fn ask_and_print_todo(&mut self) -> Result<(), TerminalError> {
        match self.ask_new_todo() {
            Ok(Some(todo)) => self.show_todo(&todo),
            Ok(None) => {
                println!("Please input a valid todo.");
                Ok(())
            }
            Err(err) => {
                println!("Error: {:?}", err);
                Ok(())
            }
        }
    }
}

fn main() {
    let mut stdin = Terminal::new();
    loop {
        if stdin.user_intention() {
            if let Ok(_) = stdin.ask_and_print_todo() {
            } else {
            }
            match stdin.ask_and_print_todo() {
                Ok(_) => (),
                Err(err) => println!("Error: {:?}", err),
            }
        } else {
            println!("Ok, quitting now.");
            break;
        }
    }
}
