use crate::domain::todo::Todo;
use std::iter::Iterator;

#[derive(PartialEq)]
pub struct Todos {
    pub list: Vec<Todo>,
}

pub struct Iter<'a> {
    todos: &'a Todos,
    index: usize,
}

impl<'a> Iterator for Iter<'a> {
    type Item = &'a Todo;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.todos.list.len() {
            let result = Some(&self.todos.list[self.index]);
            self.index += 1;
            result
        } else {
            None
        }
    }
}

impl<'a> IntoIterator for &'a Todos {
    type Item = &'a Todo;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        Iter {
            todos: self,
            index: 0,
        }
    }
}

impl Todos {
    pub fn new(list: Vec<Todo>) -> Self {
        Todos { list }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            todos: self,
            index: 0,
        }
    }

    pub fn push(&mut self, todo: Todo) {
        self.list.push(todo);
    }

    pub fn len(&mut self) -> usize {
        self.list.len()
    }

    pub fn remove(&mut self, index: usize) {
        self.list.remove(index);
    }

    pub fn get_mut(&mut self, index: usize) -> Option<&mut Todo> {
        self.list.get_mut(index)
    }

    pub fn is_empty(&self) -> bool {
        self.list.is_empty()
    }
}
