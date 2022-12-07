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
        if !self.user_intention() {
            return Ok(None);
        }

        println!("Write your new todo:");
        let user_input = self.input()?;

        if user_input.is_empty() {
            return Ok(None);
        } else {
            return Ok(Some(Todo::new(user_input)));
        }
    }

    fn show_todo(&mut self, todo: &Todo) -> Result<(), TerminalError> {
        writeln!(self.stdout, "[ ] - {}", todo.message)
            .map_err(|error| TerminalError::Stdout(error))
    }

    fn user_intention(&mut self) -> bool {
        println!("Do you want to input a new todo? (y/n)");
        let user_input = self.input();
        matches!(user_input, Ok(input) if input == "y")
    }

    fn input(&mut self) -> Result<String, TerminalError> {
        let mut buf = String::new();
        self.stdin
            .read_line(&mut buf)
            .map_err(|error| TerminalError::Stdin(error))
            .map(|_| buf.trim().to_string())
    }
}

fn run(stdin: &mut Terminal) -> Result<(), TerminalError>{
    loop {
        let todo = stdin.ask_new_todo();
        if let Ok(Some(todo)) = todo {
            stdin.show_todo(&todo)?;
        } else {
            if let Ok(None) = todo {
                println!("Ok, quitting now.");
                break;
            }
        }
    }
    Ok(())
}

fn main() {
    let mut stdin = Terminal::new();
    if let Err(error) = run(&mut stdin) {
        println!("Error: {:?}", error)
    }
}
