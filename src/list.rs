use chrono::{Local, NaiveDate};
use color_eyre::Result;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
// use std::io::{self, Write};
// use std::io::stdout;
use uuid::Uuid;

// Log:
// |
// days:Vec<Day>
// |
// +-- List
//     |
//     +-- ListItem
//     |   |
//     |   +-- id: Uuid
//     |   +-- task: String
//     |   +-- priority: Option<u32>
//     |
//     +-- order: Vec<Uuid>
// |
// +-- date: NaiveDate

#[derive(Debug, Serialize, Deserialize)]
pub struct Log {
    pub days: HashMap<NaiveDate, List>,
}

impl Log {
    pub fn new() -> Self {
        Self {
            days: HashMap::new(),
        }
    }

    pub fn add_item_to_current_day(&mut self, task: String, priority: Option<u32>) -> Result<()> {
        let today = Local::now().date_naive();

        let _ = self
            .days
            .entry(today)
            .or_insert_with(List::new)
            .add_item(task, priority);

        let json = serde_json::to_string_pretty(&self.days)?;
        fs::write("todos.json", json)?;

        Ok(())
    }

    pub fn formatted_ordered_items(self) -> String {
        let today = Local::now().date_naive();
        let mut ordered_list = String::new();
        if let Some(plist) = self.days.get(&today) {
            for (index, uuid) in plist.order.iter().enumerate() {
                if let Some(entry) = plist.todos.get(uuid) {
                    let priority_str = entry.priority.map_or("".to_string(), |p| p.to_string());
                    ordered_list.push_str(&format!(
                        "i {}, Task: {}, Priority: {}\n",
                        index, entry.task, priority_str
                    ));
                }
            }
        } else {
            println!("no entries for today")
            // Handle the case where there is no entry for today's date
        }

        ordered_list
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct List {
    pub todos: HashMap<Uuid, ListItem>,
    order: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    // #[serde(with = "uuid::serde")]
    // #[serde(with = "uuid::serde")]
    pub task: String,
    pub priority: Option<u32>,
}

impl List {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn add_item(&mut self, task: String, priority: Option<u32>) -> Result<()> {
        let new_item = ListItem { task, priority };
        let id = Uuid::new_v4();
        let _ = self.todos.insert(id, new_item);
        self.order.push(id);
        Ok(())
    }
}
