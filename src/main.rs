mod todo;
use std::io::{self, Write};
use todo::TodoList;

fn main() {
    let mut todo_list = TodoList::new("todo.txt");

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error al leer la entrada");

        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.is_empty() {
            continue;
        }

        match parts[0] {
            "add" => {
                if parts.len() > 1 {
                    let task = parts[1..].join(" ");
                    todo_list.add_task(&task);
                    println!("Tarea añadida: {}", task);
                } else {
                    println!("Uso: add <tarea>");
                }
            }
            "list" => {
                todo_list.list_tasks();
            }
            "remove" => {
                if parts.len() > 1 {
                    if let Ok(index) = parts[1].parse::<usize>() {
                        todo_list.remove_task(index - 1);
                    } else {
                        println!("Índice inválido.");
                    }
                } else {
                    println!("Uso: remove <índice>");
                }
            }
            "exit" => {
                println!("Saliendo...");
                break;
            }
            _ => {
                println!("Comando no reconocido.");
            }
        }
    }
}