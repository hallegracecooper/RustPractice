use std::io;
mod task;
mod task_manager;
use task_manager::TaskManager;

fn main() {
    let mut manager = TaskManager::new();

    loop {
        println!("\nTask Manager:");
        println!("1. Add Task");
        println!("2. List Tasks");
        println!("3. Exit");
        println!("Enter your choice: ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read input");
        let choice: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Invalid input. Please enter a number.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Enter task title:");
                let mut title = String::new();
                io::stdin().read_line(&mut title).expect("Failed to read title");

                println!("Enter task description:");
                let mut description = String::new();
                io::stdin().read_line(&mut description).expect("Failed to read description");

                manager.add_task(title.trim(), description.trim());
                println!("Task added!");
            }
            2 => {
                println!("\nTask List:");
                for task in manager.list_tasks() {
                    println!(
                        "ID: {}, Title: {}, Description: {}, Completed: {}",
                        task.id, task.title, task.description, task.completed
                    );
                }
            }
            3 => {
                println!("Exiting Task Manager.");
                break;
            }
            _ => println!("Invalid choice. Please enter a valid option."),
        }
    }
}