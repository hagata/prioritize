use chrono::{Local, NaiveDate};
use color_eyre::Result;

use color_eyre::eyre::Ok;
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

    pub fn toggle_done(&mut self, index: usize) -> Result<()> {
        let today = Local::now().date_naive();

        if let Some(list) = self.days.get_mut(&today) {
            if let Some(&id) = list.order.get(index) {
                if let Some(task) = list.todos.get_mut(&id) {
                    task.done = !task.done;
                    let json = serde_json::to_string_pretty(&self.days)?;
                    fs::write("todos.json", json)?;
                } else {
                    print!("Unable to locate task to complete");
                }
            } else {
                print!("uuid not found")
            }
        } else {
            println!("no entries for today")
            // Handle the case where there is no entry for today's date
        }
        Ok(())
    }

    pub fn formatted_ordered_items(self, pred: u32, incomplete_only: bool) -> String {
        let mut day = Local::now().date_naive();

        if pred != 0 {
            day = self.get_prev_day(pred);
            print!("\nList from, {:?}\n", day);
        }
        let mut ordered_list = String::new();

        if let Some(plist) = self.days.get(&day) {
            for (index, uuid) in plist.order.iter().enumerate() {
                if let Some(entry) = plist.todos.get(uuid) {
                    let priority_str = entry.priority.map_or("".to_string(), |p| p.to_string());

                    if !incomplete_only || incomplete_only && !entry.done {
                        ordered_list.push_str(&format!(
                            "{}-[{}] {}, {}\n",
                            index,
                            if entry.done { "x" } else { " " },
                            entry.task,
                            if !priority_str.is_empty() {
                                format!("P{}", priority_str)
                            } else {
                                "".to_string()
                            }
                        ));
                    }
                }
            }
        } else {
            println!("no entries for the day")
            // Handle the case where there is no entry for today's date
        }

        ordered_list
    }

    // move incomplete tasks from the prev_day to todays list.
    pub fn carry_over_prev(&mut self, pred: u32) -> Result<()> {
        let today = Local::now().date_naive();
        let prev_day = self.get_prev_day(pred);
        print!("today {:?}, prev {:?}", today, prev_day);

        if let Some(prev_list) = self.days.get(&prev_day) {
            let incomplete_tasks: HashMap<Uuid, ListItem> = prev_list
                .todos
                .iter()
                .filter(|(_uuid, item)| !item.done)
                .map(|(&uuid, item)| (uuid, item.clone())) // Clone each item and create a tuple
                .collect();

            dbg!(&incomplete_tasks);

            // Append to today's entry or create a new entry with the completed tasks
            self.days
                .entry(today)
                .or_insert_with(|| List::new())
                .todos
                .extend(incomplete_tasks.clone());

            print!(
                "\n Carried over {} task{}\n",
                incomplete_tasks.len(),
                if incomplete_tasks.len() > 1 { "s" } else { "" }
            );

            // append the order to the end of the current list
            let ordering: Vec<Uuid> = incomplete_tasks.keys().cloned().collect();

            self.days
                .entry(today)
                .or_insert_with(List::new)
                .order
                .extend(ordering);

            let json = serde_json::to_string_pretty(&self.days)?;
            fs::write("todos.json", json)?;
        } else {
            println!("Error finding prev list and carrying over tasks");
        }
        Ok(())
    }

    // Get the last entry before _today_, generally "yesterday", but
    // will search back to the last date with an entry
    fn get_prev_day(&self, pred: u32) -> NaiveDate {
        let mut day = Local::now().date_naive();

        for _i in 0..pred {
            day = day.pred_opt().unwrap();
            let mut last_list = false;
            while !last_list {
                if let Some(_) = self.days.get(&day) {
                    last_list = true;
                } else {
                    day = day.pred_opt().unwrap();
                }
            }
        }

        day
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct List {
    pub todos: HashMap<Uuid, ListItem>,
    order: Vec<Uuid>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
    // #[serde(with = "uuid::serde")]
    // #[serde(with = "uuid::serde")]
    pub task: String,
    pub priority: Option<u32>,
    pub done: bool,
}

impl List {
    pub fn new() -> Self {
        Self {
            todos: HashMap::new(),
            order: Vec::new(),
        }
    }

    pub fn add_item(&mut self, task: String, priority: Option<u32>) -> Result<()> {
        let new_item = ListItem {
            task,
            priority,
            done: false,
        };
        let id = Uuid::new_v4();
        let _ = self.todos.insert(id, new_item);
        self.order.push(id);
        Ok(())
    }
}
