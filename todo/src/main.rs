use ::std::io::{Stdin, Stdout, Write};

struct Terminal {
    stdin: Stdin,
    stdout: Stdout,
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

    fn ask_new_todo(&mut self) -> Todo {
        println!("Write your new todo:");
        Todo::new(Self::input(self))
    }

    fn show_todo(&mut self, todo: &Todo) {
        println!("New todo added!");
        writeln!(self.stdout, "[ ] - {}", todo.message).unwrap()
    }

    fn ask_user_intention(&mut self) -> String {
        println!("Do you want to input a new todo? (y/n)");
        return Self::input(self)
    }

    fn input(&mut self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}

fn main() {
    loop {
        let mut stdin = Terminal::new();
        
        if stdin.ask_user_intention() == "y" {
            let new_todo = stdin.ask_new_todo();
            stdin.show_todo(&new_todo);
        } else {
            println!("Ok, quitting now.");
            std::process::exit(0);
        }
    }
}
