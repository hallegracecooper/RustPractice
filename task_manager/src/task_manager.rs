use crate::task::Task;

pub struct TaskManager {
    pub tasks: Vec<Task>,
}

impl TaskManager {
    pub fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, title: &str, description: &str) {
        let id = self.tasks.len() + 1;
        let task = Task::new(id, title, description);
        self.tasks.push(task);
    }

    pub fn list_tasks(&self) -> &Vec<Task> {
        &self.tasks
    }

    pub fn update_task(&mut self, id: usize, new_title: &str, new_description: &str) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.title = new_title.to_string();
            task.description = new_description.to_string();
        }
    }

    pub fn delete_task(&mut self, id: usize) {
        self.tasks.retain(|task| task.id != id);
    }

    pub fn complete_task(&mut self, id: usize) {
        if let Some(task) = self.tasks.iter_mut().find(|t| t.id == id) {
            task.mark_complete();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Test Task", "This is a test");

        assert_eq!(manager.tasks.len(), 1);
        assert_eq!(manager.tasks[0].title, "Test Task");
    }

    #[test]
    fn test_update_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Old Title", "Old Description");

        manager.update_task(1, "New Title", "New Description");

        assert_eq!(manager.tasks[0].title, "New Title");
        assert_eq!(manager.tasks[0].description, "New Description");
    }

    #[test]
    fn test_delete_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Task to Delete", "Should be removed");

        manager.delete_task(1);
        assert_eq!(manager.tasks.len(), 0);
    }

    #[test]
    fn test_complete_task() {
        let mut manager = TaskManager::new();
        manager.add_task("Task to Complete", "Should be marked done");

        manager.complete_task(1);
        assert!(manager.tasks[0].completed);
    }
}