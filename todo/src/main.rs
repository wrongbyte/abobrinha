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
        Todo::new(self.input())
    }

    fn show_todo(&mut self, todo: &Todo) {
        writeln!(self.stdout, "[ ] - {}", todo.message).unwrap()
    }

    fn user_intention(&mut self) -> bool {
        println!("Do you want to input a new todo? (y/n)");
        if Self::input(self) == "y" {
            return true;
        }
        false
    }

    fn input(&mut self) -> String {
        let mut buf = String::new();
        self.stdin.read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}

fn main() {
    let mut stdin = Terminal::new();
    loop {
        if stdin.user_intention() {
            let new_todo = stdin.ask_new_todo();
            stdin.show_todo(&new_todo);
        } else {
            println!("Ok, quitting now.");
            break;
        }
    }
}
