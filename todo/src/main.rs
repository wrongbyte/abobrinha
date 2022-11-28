use std::io;

fn main() {
    ask_user_input();
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn ask_user_input() {
    let mut new_todo = String::new();
    println!("Do you want to input a new todo? (y/n)");
    if input() == "y" {
        println!("Write your new todo:");
        io::stdin().read_line(&mut new_todo).unwrap();
        new_todo.trim().to_string();

        println!("New todo added!");
        println!(" [ ] - {}", new_todo);
        return ask_user_input()
    } 
    println!("Ok, quitting now.")
}
