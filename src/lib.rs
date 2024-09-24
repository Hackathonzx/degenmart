use alloc::{string::String, vec::Vec};
use stylus_sdk::{alloy_primitives::U256, prelude::*, msg, storage::StorageMap};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Task {
    id: U256,
    description: String,
    reward: U256,
    creator: Address,
    assignee: Option<Address>,
    completed: bool,
}

#[stylus_sdk::contract]
mod task_marketplace {
    use super::*;

    #[storage]
    struct TaskMarketplace {
        tasks: StorageMap<U256, Task>,
        task_count: U256,
    }

    #[external]
    impl TaskMarketplace {
        pub fn create_task(&mut self, description: String, reward: U256) -> Result<U256, Vec<u8>> {
            let task_id = self.task_count.add(1);
            let task = Task {
                id: task_id,
                description,
                reward,
                creator: msg::sender(),
                assignee: None,
                completed: false,
            };
            self.tasks.insert(task_id, task);
            self.task_count = task_id;
            Ok(task_id)
        }

        pub fn accept_task(&mut self, task_id: U256) -> Result<(), Vec<u8>> {
            let mut task = self.tasks.get(task_id).ok_or("Task not found")?;
            if task.assignee.is_some() {
                return Err("Task already assigned".into());
            }
            task.assignee = Some(msg::sender());
            self.tasks.insert(task_id, task);
            Ok(())
        }

        pub fn complete_task(&mut self, task_id: U256) -> Result<(), Vec<u8>> {
            let mut task = self.tasks.get(task_id).ok_or("Task not found")?;
            if task.assignee != Some(msg::sender()) {
                return Err("Not assigned to this task".into());
            }
            if task.completed {
                return Err("Task already completed".into());
            }
            task.completed = true;
            self.tasks.insert(task_id, task.clone());
            
            // Transfer reward to assignee
            msg::send(task.assignee.unwrap(), task.reward);
            Ok(())
        }

        pub fn get_task(&self, task_id: U256) -> Result<Task, Vec<u8>> {
            self.tasks.get(task_id).ok_or("Task not found".into())
        }

        pub fn get_all_tasks(&self) -> Vec<Task> {
            (1..=self.task_count)
                .filter_map(|id| self.tasks.get(id))
                .collect()
        }
    }
}