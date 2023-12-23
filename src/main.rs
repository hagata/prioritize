use chrono::Local;
use color_eyre::Result;
use serde_json;
use std::path::Path;

// use serde::{Deserialize, Serialize};
use std::fs;

use clap::{Parser, Subcommand};

//
//local
//
mod list;
use list::Log;
mod ui;
// use ui::_launch_ui;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    // #[arg(short, long, action = clap::ArgAction::Count)]
    // debug: u8,
    //
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// Handle task prioritization
    Prioritize {
        #[command(subcommand)]
        action: PrioritizeActions,
    },
}

#[derive(Subcommand)]
enum PrioritizeActions {
    // add a new task with optional priority
    Add {
        #[arg(required = true)]
        task: String,

        #[arg(short, long)]
        priority: Option<u32>,
    },

    List {},
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut main_list = Log::new();
    // Load existing items from the file
    let path = Path::new("todos.json");
    if path.exists() {
        let file = fs::File::open(path)?;
        main_list.days = serde_json::from_reader(file)?;
    }

    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Prioritize { action }) => match action {
            PrioritizeActions::Add { task, priority } => {
                // todo!();
                // let priority = Some(priority);
                let _ = main_list.add_item_to_current_day(task.clone(), *priority);
                // Serialize the updated list and write it to the file
                dbg!(&main_list);
            }
            PrioritizeActions::List {} => {
                let today = Local::now().date_naive();
                if let Some(plist) = main_list.days.get(&today) {
                    for entry in &plist.todos {
                        //     let Some(String::priority) = entry.priority else {
                        //         ""                    }
                        println!("{:?} {:?}", entry.priority, entry.task);
                    }
                } else {
                    println!("no entries for today")
                    // Handle the case where there is no entry for today's date
                }
            }
        },
        // start the UI when no commands are passed
        None => {
            println!("No command ");
            //launch_ui();
            todo!("convert the old loop into a mod with a better interface")
        }
    }

    Ok(())

    // Continued program logic goes here...
}
