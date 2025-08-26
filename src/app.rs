use chrono::Local;
use serde::{Deserialize, Serialize};
use crate::data::{App, ItemStatus, TodoItem, TodoList};
use crate::storage::Storage;

impl App {
    pub fn load() -> Self {
        let storage = Storage::get();
        let json_data = storage.load();
        let lists:App = serde_json::from_str(&json_data)
            .expect("🦀🦀 App Data seems to be corrupted");
        lists
    }
    
    pub fn save(&self) {
        let storage = Storage::get();
        let json_data = serde_json::to_string_pretty(&self.lists).expect("Failed to serialize data");
        storage.save(json_data);
    }

    pub fn create_new_list(&mut self, name: String) {
        if self.lists.iter().any(|l| l.name == name) {
            println!("❌ A list named '{}' already exists.", name);
            return;
        }

        let new_list = TodoList::new(name);
        self.lists.push(new_list);
        self.save();
        println!("✅ Created new list.");
    }
    
    pub fn list_all_lists(&self) -> String {
        let lists = &self.lists;
        lists.iter().enumerate()
            .map(|(i, list)| format!("{}. {}", i + 1, list.name))
            .collect::<Vec<String>>()
            .join("\n")
    }

    pub fn delete_list(&mut self, id_or_name: &str) {
        // Try parsing as index
        if let Ok(index) = id_or_name.parse::<usize>() {
            if index == 0 || index > self.lists.len() {
                println!("❌ Invalid list number.");
                return;
            }
            let removed = self.lists.remove(index - 1);
            println!("🗑️ Deleted list: {}", removed.name);
        } else {
            // Try finding by name
            if let Some(pos) = self.lists.iter().position(|l| l.name == id_or_name) {
                let removed = self.lists.remove(pos);
                println!("🗑️ Deleted list: {}", removed.name);
            } else {
                println!("❌ No list found with name '{}'.", id_or_name);
            }
        }
        self.save();
    }

    pub fn add_todo(&mut self, name: String, description: String) {
        let list_name = match &self.active_list_name {
            Some(name) => name,
            None => {
                println!("❗ No active list selected. Use `switch <list>` to select one.");
                return;
            }
        };

        // Try to find the active list
        if let Some(list) = self.lists.iter_mut().find(|l| &l.name == list_name) {
            list.items.push(TodoItem::new(name, description));
            self.save();
            println!("✅ Todo added to '{}'", list_name);
        } else {
            println!("⚠️ Please select a list");
            self.active_list_name = None;
        }
        self.save();
    }
    
    pub fn remove_todo(&mut self, id_or_name: &str) {
        let list_name = match &self.active_list_name {
            Some(name) => name,
            None => {
                println!("⚠️ Please select a list");
                return;
            }
        };
        let list = match self.lists.iter_mut().find(|l| &l.name == list_name) {
            Some(list) => list,
            None => {
                println!("⚠️ Please select a list");
                self.active_list_name = None;
                return;
            }
        };
        self.save()
    }

    pub fn update_status(&mut self, id_or_name: &str, new_status: &str) {
        // 1. Check active list
        let list_name = match &self.active_list_name {
            Some(name) => name,
            None => {
                println!("❗ No active list selected. Use 'switch <list>' to select one.");
                return;
            }
        };

        // 2. Find the list
        let list = match self.lists.iter_mut().find(|l| &l.name == list_name) {
            Some(list) => list,
            None => {
                println!("⚠️ Active list '{}' not found. It might have been deleted.", list_name);
                self.active_list_name = None;
                return;
            }
        };

        // 3. Find the todo item
        let index = if let Ok(num) = id_or_name.parse::<usize>() {
            if num == 0 || num > list.items.len() {
                println!("❌ Invalid task number.");
                return;
            }
            num - 1
        } else {
            match list.items.iter().position(|item| item.name == id_or_name) {
                Some(pos) => pos,
                None => {
                    println!("❌ No task found with name or ID '{}'.", id_or_name);
                    return;
                }
            }
        };

        // 4. Parse the new status
        let status = match new_status.to_lowercase().as_str() {
            "todo" => ItemStatus::Todo {
                created_at: Local::now(),
            },
            "ongoing" => ItemStatus::Ongoing {
                started_at: Local::now(),
            },
            "done" => ItemStatus::Done {
                completed_at: Local::now(),
            },
            _ => {
                println!("❌ Invalid status: '{}'. Use 'todo', 'ongoing', or 'done'.", new_status);
                return;
            }
        };

        // 5. Update the status
        list.items[index].status = status;
        println!("✅ Updated status of '{}'.", list.items[index].name);
        self.save();
    }
    
}

pub fn about()->String {
    "
      _____     ____   ___            _   _                 _                 __ _           _   _         ____            _   
 |_   _|__ |  _ \\ / _ \\          | | | | __ _ _ __   __| | ___ _ __ __ _ / _| |_ ___  __| | (_)_ __   |  _ \\ _   _ ___| |_ 
   | |/ _ \\| | | | | | |  _____  | |_| |/ _` | '_ \\ / _` |/ __| '__/ _` | |_| __/ _ \\/ _` | | | '_ \\  | |_) | | | / __| __|
   | | (_) | |_| | |_| | |_____| |  _  | (_| | | | | (_| | (__| | | (_| |  _| ||  __/ (_| | | | | | | |  _ <| |_| \\__ \\ |_ 
   |_|\\___/|____/ \\___/          |_| |_|\\__,_|_| |_|\\__,_|\\___|_|  \\__,_|_|  \\__\\___|\\__,_| |_|_| |_| |_| \\_\\__,_|___/\\__|
                                                                                                                           
    ".to_string()
}