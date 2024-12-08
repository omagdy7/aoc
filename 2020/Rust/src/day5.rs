use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, Write},
    sync::{Arc, Mutex},
    thread,
};

#[derive(Debug, Clone)]
enum TaskStatus {
    Pending,
    Completed,
}

#[derive(Debug, Clone)]
struct Task {
    id: usize,
    description: String,
    status: TaskStatus,
}

impl Task {
    fn new(id: usize, description: String) -> Self {
        Self {
            id,
            description,
            status: TaskStatus::Pending,
        }
    }

    fn mark_done(&mut self) {
        self.status = TaskStatus::Completed;
    }

    fn to_string(&self) -> String {
        format!(
            "{}|{}|{}",
            self.id,
            self.description,
            match self.status {
                TaskStatus::Pending => "Pending",
                TaskStatus::Completed => "Completed",
            }
        )
    }

    fn from_string(data: &str) -> Option<Self> {
        let parts: Vec<&str> = data.split('|').collect();
        if parts.len() != 3 {
            return None;
        }
        let id = parts[0].parse().ok()?;
        let description = parts[1].to_string();
        let status = match parts[2] {
            "Pending" => TaskStatus::Pending,
            "Completed" => TaskStatus::Completed,
            _ => return None,
        };

        Some(Self {
            id,
            description,
            status,
        })
    }
}

#[derive(Debug)]
struct TaskManager {
    tasks: Vec<Task>,
}

impl TaskManager {
    fn new() -> Self {
        Self { tasks: vec![] }
    }

    fn add_task(&mut self, description: String) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, description);
        self.tasks.push(task);
    }

    fn mark_task_done(&mut self, id: usize) -> Result<(), String> {
        match self.tasks.iter_mut().find(|t| t.id == id) {
            Some(task) => {
                task.mark_done();
                Ok(())
            }
            None => Err(format!("Task with ID {} not found.", id)),
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            println!(
                "ID: {}, Description: {}, Status: {:?}",
                task.id, task.description, task.status
            );
        }
    }

    fn save_to_file(&self, filename: &str) -> io::Result<()> {
        let mut file = File::create(filename)?;
        for task in &self.tasks {
            writeln!(file, "{}", task.to_string())?;
        }
        Ok(())
    }

    fn load_from_file(&mut self, filename: &str) -> io::Result<()> {
        self.tasks.clear();
        let file = File::open(filename)?;
        let reader = BufReader::new(file);
        for line in reader.lines() {
            if let Ok(line) = line {
                if let Some(task) = Task::from_string(&line) {
                    self.tasks.push(task);
                }
            }
        }
        Ok(())
    }
}

fn add(x: i32, y: i32, z: i32) -> i32 {
    x + y + z
}

fn main() {
    let task_manager = Arc::new(Mutex::new(TaskManager::new()));

    let xay = 5;
    let yay = 6;
    let x = xay + yay + add(xay, yay, 3);
    let y = 2;

    // let z = add(x, y);

    let tm_clone = Arc::clone(&task_manager);
    println!(
        "tm_clone: {:?}, Arc::clone(&task_manager): {:?}",
        tm_clone,
        Arc::clone(&task_manager)
    );
    thread::spawn(move || {
        let mut tm = tm_clone.lock().unwrap();
        tm.add_task("Learn Rust".to_string());
        tm.add_task("Build a project".to_string());
        tm.list_tasks();
    })
    .join()
    .unwrap();

    let tm_clone = Arc::clone(&task_manager);
    thread::spawn(move || {
        let mut tm = tm_clone.lock().unwrap();
        tm.mark_task_done(1)
            .unwrap_or_else(|err| println!("{}", err));
        tm.list_tasks();
    })
    .join()
    .unwrap();

    let tm_clone = Arc::clone(&task_manager);
    thread::spawn(move || {
        let mut tm = tm_clone.lock().unwrap();
        let filename = "tasks.txt";
        tm.save_to_file(filename).expect("Failed to save tasks");
        tm.load_from_file(filename).expect("Failed to load tasks");
        tm.list_tasks();
    })
    .join()
    .unwrap();
}
