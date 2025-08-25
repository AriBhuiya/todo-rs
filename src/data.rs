use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Lists{
    pub list: Vec<TodoList>
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoList{
    pub name: String,
    created_at: DateTime<Local>,
    items: Vec<TodoItem>,
}

impl TodoList {
    pub fn new(name:String) -> TodoList {
        TodoList{
            name,
            created_at: Local::now(),
            items: vec![],
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub struct TodoItem {
    name: String,
    description: String,
    status: ItemStatus,
}

impl TodoItem {
    pub fn new(name: String, description: String) -> TodoItem {
        TodoItem{
            name,
            description,
            status:ItemStatus::Todo {created_at: Local::now()},
        }
    }
}
#[derive(Serialize, Deserialize, Debug)]
pub enum ItemStatus {
    Todo {created_at: DateTime<Local>},
    Ongoing { started_at: DateTime<Local> },
    Done { completed_at: DateTime<Local> },
}

impl ItemStatus {
}