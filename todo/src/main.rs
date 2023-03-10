use crate::terminal::Terminal;
use controllers::todo::{TodoController, TodoControllerImpl};
use db::connect::connect;
use repository::todo::PostgresTodoRepository;
use terminal::{error::TerminalError, UserInterface, UserOptions};
mod controllers;
mod db;
mod domain;
mod repository;
mod terminal;
extern crate factori;

#[tokio::main]
async fn main() {
    let client = connect()
        .await
        .expect("Database connection error. Quitting");
    let todo_repository = Box::new(PostgresTodoRepository { client });
    let user_interface = Box::new(Terminal::new());

    let mut todo_controller = Box::new(TodoControllerImpl {
        todo_repository,
        user_interface,
    });

    loop {
        if let Err(error) =
            get_user_intention(&mut Box::new(Terminal::new()), &mut todo_controller).await
        {
            println!("{}", &error);
            if error.is_fatal() {
                break;
            }
        }
    }
}

async fn get_user_intention(
    user_interface: &mut Box<Terminal>,
    todo_controller: &mut Box<TodoControllerImpl>,
) -> Result<(), TerminalError> {
    loop {
        match user_interface.user_intention()? {
            UserOptions::Quit => break,
            UserOptions::NewTodo(todo) => todo_controller.add_todo(todo).await?,
            UserOptions::Help => user_interface.show_help()?,
            UserOptions::ClearList => todo_controller.clear_todo_list().await?,
            UserOptions::RemoveTodo(index) => todo_controller.remove_todo(index).await?,
            UserOptions::Unrecognized => user_interface.alert_unrecognized()?,
            UserOptions::ShowList => todo_controller.show_list().await?,
            UserOptions::DoTodo(index) => todo_controller.mark_todo_done(index).await?,
        }
    }
    user_interface.write_interface(&"Ok, quitting now.")?;
    Ok(())
}

pub mod mocks {
    use crate::domain::todo::Todo;
    use crate::domain::todos::Todos;
    use uuid::Uuid;

    pub fn builder(number_todos: usize, done_todo: Option<usize>) -> Todos {
        let list: Vec<Todo> = (0..number_todos)
            .map(|index| {
                let id = Uuid::new_v4();
                let message = format!("todo {}", index);
                let mut todo = Todo::new(message.to_string(), id);
                if let Some(done_index) = done_todo {
                    if index == done_index {
                        todo.done = true;
                    }
                }
                todo
            })
            .collect();
        Todos::new(list)
    }

    factori::factori!(Todos, {
        default {
            _list:Vec<Todo> = vec![],
            number_todos: usize = 0,
            done_todo: Option<usize> = None
        }
        builder {
            builder(number_todos, done_todo)
        }
    });
}
