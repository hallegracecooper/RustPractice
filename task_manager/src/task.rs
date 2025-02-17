#[derive(Debug, Clone)]
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: String,
    pub completed: bool,
}

impl Task {
    pub fn new(id: usize, title: &str, description: &str) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            completed: false,
        }
    }

    pub fn mark_complete(&mut self) {
        self.completed = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_task_creation() {
        let task = Task::new(1, "Test Task", "This is a test");
        assert_eq!(task.id, 1);
        assert_eq!(task.title, "Test Task");
        assert_eq!(task.description, "This is a test");
        assert!(!task.completed);
    }

    #[test]
    fn test_mark_complete() {
        let mut task = Task::new(1, "Test Task", "This is a test");
        task.mark_complete();
        assert!(task.completed);
    }
}