mod todo;

use std::env;
use todo::TodoList;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        return;
    }

    let mut list = TodoList::new();

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Usage: todo-app add <title>");
                return;
            }
            let title = args[2..].join(" ");
            let id = list.add(&title);
            println!("Added: [{}] {}", id, title);
        }
        "done" => {
            if args.len() < 3 {
                eprintln!("Usage: todo-app done <id>");
                return;
            }
            let id: usize = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Invalid id: {}", args[2]);
                    return;
                }
            };
            if list.done(id) {
                println!("Marked as done: {}", id);
            } else {
                println!("Not found: {}", id);
            }
        }
        "remove" => {
            if args.len() < 3 {
                eprintln!("Usage: todo-app remove <id>");
                return;
            }
            let id: usize = match args[2].parse() {
                Ok(id) => id,
                Err(_) => {
                    eprintln!("Invalid id: {}", args[2]);
                    return;
                }
            };
            if list.remove(id) {
                println!("Removed: {}", id);
            } else {
                println!("Not found: {}", id);
            }
        }
        "list" => {
            let todos = list.list();
            if todos.is_empty() {
                println!("No todos.");
            } else {
                for t in todos {
                    let status = if t.done { "x" } else { " " };
                    println!("[{}] [{}] {}", t.id, status, t.title);
                }
            }
        }
        "count" => {
            let count = list.list().len();
            println!("{}", count);
        }
        "clear" => {
            list.clear();
            println!("All todos cleared.");
        }
        _ => {
            print_usage();
        }
    }
}

fn print_usage() {
    eprintln!("Usage: todo-app <command> [args]");
    eprintln!();
    eprintln!("Commands:");
    eprintln!("  add <title>   Add a new todo");
    eprintln!("  done <id>     Mark a todo as done");
    eprintln!("  remove <id>   Remove a todo");
    eprintln!("  list          List all todos");
    eprintln!("  count         Show total number of todos");
}
