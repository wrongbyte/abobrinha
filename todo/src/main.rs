fn main() {
    ask_user_input();
}

fn input() -> String {
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

fn ask_user_input() {
    println!("Do you want to input a new todo? (y/n)");
    if input() == "y" {
        println!("Write your new todo:");
        let new_todo = input();
        println!("New todo added!");
        println!(" [ ] - {}", new_todo);
        return ask_user_input()
    } 
    println!("Ok, quitting now.")
}
