use crate::terminal::Terminal;
use controllers::todo::{TodoController, TodoControllerImpl};
use db::connect::connect;
use repository::todo::PostgresTodoRepository;
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
        if let Err(error) = todo_controller.get_user_intention().await {
            todo_controller.user_interface.print_error(&error);
            if error.is_fatal() {
                break;
            }
        }
    }
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
                let mut todo = Todo::new(message, id);
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
