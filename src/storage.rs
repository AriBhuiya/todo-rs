use std::fs::{self, File};
use std::io::Write;

pub struct Storage {
    pub name: String,
    pub location: String,
}

impl Storage {
    pub fn get(initial_data_if_not_exists:String) -> Self {
        let name = "data";
        let home_dir = dirs::home_dir().expect("Could not find home directory");
        let dir_path = home_dir.join(".todo");

        // check ~/.to-do exists
        if !dir_path.exists() {
            fs::create_dir_all(&dir_path).expect("Failed to create ~/.todo directory");
        }

        // Build the file path
        let file_path = dir_path.join(format!("{}.todo", name));

        // Create file if it doesn't exist
        if !file_path.exists() {
            let mut file = File::create(&file_path).expect("Failed to create todo file");
            file.write_all(initial_data_if_not_exists).expect("Failed to write initial contents");
        }

        Self {
            name: name.to_string(),
            location: file_path.to_string_lossy().to_string(),
        }
    }

    pub fn save(&self, data: String) {
        fs::write(&self.location, data).expect("Failed to save data");
    }

    pub fn load(&self) -> String {
        fs::read_to_string(&self.location).unwrap_or_else(|_| "[]".to_string())
    }
}