use color_eyre::Result;
use serde_json;
use std::path::Path;

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

    Done {
        #[arg(required = true)]
        index: String,
    },

    #[command(alias = "ls")]
    List {
        #[arg(short, long)]
        pred: Option<u32>,
    },
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
                // print the updated list
                let ordered_list = main_list.formatted_ordered_items(0u32);
                print!("{}", ordered_list);
            }

            PrioritizeActions::Done { index } => {
                let _ = main_list.mark_done(index.parse::<usize>().unwrap());
                let ordered_list = main_list.formatted_ordered_items(0);
                print!("\n\n{}\n", ordered_list);
            }
            PrioritizeActions::List { pred } => {
                let pred = pred.unwrap_or(0u32);
                let ordered_list = main_list.formatted_ordered_items(pred);
                print!("\n\n{}\n", ordered_list);
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
