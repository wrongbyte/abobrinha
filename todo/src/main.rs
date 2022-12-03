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

    fn ask_new_todo(&mut self) -> Option<Todo> {
        println!("Write your new todo:");
        match self.input() {
            Ok(user_input) => {
                if user_input.is_empty() {
                    return None;
                }
                Some(Todo::new(user_input))
            },
            Err(_) => None
        }
 
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        match writeln!(self.stdout, "[ ] - {}", todo.message) {
            Ok(_) => Ok(()),
            Err(err) => Err(TerminalError::Stdout(err))
        }
    }

    fn user_intention(&mut self) -> bool {
        println!("Do you want to input a new todo? (y/n)");
        let user_input = self.input();
        if let Ok(input) = user_input {
            return input == "y"
        } 
        false
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        match self.stdin.read_line(&mut buf) {
            Ok(_) => Ok(buf.trim().to_string()),
            Err(error) => Err(TerminalError::Stdin(error)),
        }
    }

    fn ask_and_print_todo(&mut self) -> Result<(), TerminalError> {
        match self.ask_new_todo() {
            Some(todo) => Ok(self.show_todo(&todo))?,
            None => Ok(println!("Please input a valid todo.")),
        }
    }
}

fn main() {
    let mut stdin = Terminal::new();
    loop {
        if stdin.user_intention() {
            match stdin.ask_and_print_todo() {
                Ok(_) => (),
                Err(err) => println!("Error: {:?}", err)
            }
        } else {
            println!("Ok, quitting now.");
            break;
        }
    }
}
