#[derive(Debug, Clone)]
pub struct Todo {
    pub id: usize,
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Default)]
pub struct TodoList {
    todos: Vec<Todo>,
    next_id: usize,
}

impl TodoList {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add(&mut self, title: &str) -> usize {
        let id = self.next_id;
        self.todos.push(Todo {
            id,
            title: title.to_string(),
            done: false,
        });
        self.next_id += 1;
        id
    }

    pub fn done(&mut self, id: usize) -> bool {
        if let Some(todo) = self.todos.iter_mut().find(|t| t.id == id) {
            todo.done = true;
            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, id: usize) -> bool {
        let len = self.todos.len();
        self.todos.retain(|t| t.id != id);
        self.todos.len() != len
    }

    pub fn list(&self) -> &[Todo] {
        &self.todos
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add() {
        let mut list = TodoList::new();
        let id = list.add("Buy milk");
        assert_eq!(id, 1);
        assert_eq!(list.list().len(), 1);
        assert_eq!(list.list()[0].title, "Buy milk");
        assert!(!list.list()[0].done);
    }

    #[test]
    fn test_add_multiple() {
        let mut list = TodoList::new();
        let id1 = list.add("Task 1");
        let id2 = list.add("Task 2");
        assert_eq!(id1, 1);
        assert_eq!(id2, 2);
        assert_eq!(list.list().len(), 2);
    }

    #[test]
    fn test_done() {
        let mut list = TodoList::new();
        list.add("Buy milk");
        assert!(list.done(1));
        assert!(list.list()[0].done);
    }

    #[test]
    fn test_done_not_found() {
        let mut list = TodoList::new();
        assert!(!list.done(99));
    }

    #[test]
    fn test_remove() {
        let mut list = TodoList::new();
        list.add("Buy milk");
        assert!(list.remove(1));
        assert!(list.list().is_empty());
    }

    #[test]
    fn test_remove_not_found() {
        let mut list = TodoList::new();
        assert!(!list.remove(99));
    }

    #[test]
    fn test_list_empty() {
        let list = TodoList::new();
        assert!(list.list().is_empty());
    }
}
