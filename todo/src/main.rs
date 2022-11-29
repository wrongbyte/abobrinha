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
        Todo::new(Self::input())
    }

    fn show_todo(&mut self, todo: &Todo) {
        println!("New todo added!");
        writeln!(self.stdout, "[ ] - {}", todo.message).unwrap()
    }

    fn input() -> String {
        let mut buf = String::new();
        std::io::stdin().read_line(&mut buf).unwrap();
        buf.trim().to_string()
    }
}

fn main() {
    ask_user_input();
}

fn ask_user_input() {
    println!("Do you want to input a new todo? (y/n)");
    let mut stdin = Terminal::new();
    
    if Terminal::input() == "y" {
        let new_todo = stdin.ask_new_todo();
        stdin.show_todo(&new_todo);
        return ask_user_input();
    }
    println!("Ok, quitting now.");
    std::process::exit(0);
}
