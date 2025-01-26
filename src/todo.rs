use std::fs::{File, OpenOptions};
use std::io::{self, Write, BufRead, BufReader};
use std::path::Path;

#[derive(Debug)]
pub struct TodoList {
    tasks: Vec<String>,
    file_path: String,
}

impl TodoList {
    pub fn new(file_path: &str) -> Self {
        let tasks = if Path::new(file_path).exists() {
            let file = File::open(file_path).expect("No se pudo abrir el archivo");
            let reader = BufReader::new(file);
            reader.lines().map(|line| line.unwrap()).collect()
        } else {
            Vec::new()
        };

        TodoList {
            tasks,
            file_path: file_path.to_string(),
        }
    }

    pub fn add_task(&mut self, task: &str) {
        self.tasks.push(task.to_string());
        self.save_tasks();
    }

    pub fn list_tasks(&self) {
        if self.tasks.is_empty() {
            println!("No hay tareas pendientes.");
        } else {
            for (index, task) in self.tasks.iter().enumerate() {
                println!("{}: {}", index + 1, task);
            }
        }
    }

    pub fn remove_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks.remove(index);
            self.save_tasks();
        } else {
            println!("Índice inválido.");
        }
    }

    fn save_tasks(&self) {
        let mut file = OpenOptions::new()
            .write(true)
            .truncate(true)
            .create(true)
            .open(&self.file_path)
            .expect("No se pudo abrir el archivo");

        for task in &self.tasks {
            writeln!(file, "{}", task).expect("No se pudo escribir en el archivo");
        }
    }
}