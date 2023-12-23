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
}

// impl Day {
//     pub fn new(date: NaiveDate) -> Self {
//         Self {
//             date,
//             list: List::new(),
//         }
//     }
//
//     pub fn add_item(&mut self, task: String, priority: Option<u32>) {
//         let _ = self.list.add_item(task, priority);
//     }
// }

// #[derive(Debug, Serialize, Deserialize)]
// pub struct Day {
//     pub date: NaiveDate,
//     pub list: List,
// }

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct List {
    pub todos: Vec<ListItem>,
    order: Vec<Uuid>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListItem {
    // #[serde(with = "uuid::serde")]
    // #[serde(with = "uuid::serde")]
    id: Uuid,
    pub task: String,
    pub priority: Option<u32>,
}

impl List {
    pub fn new() -> Self {
        Self {
            todos: Vec::new(),
            order: Vec::new(),
        }
    }

    pub fn add_item(&mut self, task: String, priority: Option<u32>) -> Result<()> {
        let new_item = ListItem {
            id: Uuid::new_v4(), // Generate a unique ID
            task,
            priority,
        };
        self.todos.push(new_item); // Add item to todos list
        Ok(())
    }
}
