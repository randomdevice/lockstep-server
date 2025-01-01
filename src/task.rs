use uuid::Uuid;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum TaskState {
    Expired,
    Active,
    Scheduled,
    Idle
}

#[derive(Debug, Clone, Eq)]
pub struct Task {
    pub uuid: Uuid,
    name: String,
    description: String,
    state: TaskState, // state of the task, based on current active slot
    slot_range: u8, // amount of slots a Task occupies, min 1, max dependentent on Timeset
    start_slot: u8 // the slot a task is initially scheduled on
}

impl PartialEq for Task {
    fn eq(&self, other: &Self) -> bool {
        self.uuid == other.uuid
    }
}

impl Default for Task {
    fn default() -> Self {
        Self {
            uuid: Uuid::now_v7(),
            name: "Idle".to_string(),
            description: "A task with no purpose.".to_string(),
            state: TaskState::Idle,
            slot_range: 24,
            start_slot: 0
        }
    }
}

