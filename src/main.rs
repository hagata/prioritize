use color_eyre::Result;
use serde_json;
use std::fs;
use std::path::Path;
#[cfg(debug_assertions)]
use std::time::Instant;

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

    #[command(about = "Remove a task completely from the list")]
    Remove {
        #[arg(required = true)]
        index: String,
    },

    #[command(about = "Mark a task as done")]
    Done {
        #[arg(required = true)]
        index: String,
    },

    #[command(alias = "ls")]
    List {
        #[arg(
            short,
            long,
            help = "previous n of days-ago to list. Will show the last day with tasks"
        )]
        pred: Option<u32>,

        #[arg(short, long, help = "only show incomplete tasks")]
        incomplete: bool,
    },

    #[command(about = "carry over incomplete tasks from the previous day to todays list")]
    Carry {
        #[arg(
            short,
            long,
            help = "previous n of days-ago to carry over incomplete tasks from. skips empty/missing days."
        )]
        pred: Option<u32>,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut main_list = Log::new();
    #[cfg(debug_assertions)]
    let start = Instant::now();
    // Load existing items from the file
    let path = Path::new("todos.json");
    if path.exists() {
        let file = fs::File::open(path)?;
        main_list.days = serde_json::from_reader(file)?;
    }
    #[cfg(debug_assertions)]
    println!("Loading todos took {:?}", start.elapsed());

    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Prioritize { action }) => match action {
            PrioritizeActions::Add { task, priority } => {
                // todo!();
                // let priority = Some(priority);
                let _ = main_list.add_item_to_current_day(task.clone(), *priority);
                // print the updated list
                let ordered_list = main_list.formatted_ordered_items(0u32, false);
                println!("\n\n{}\n", ordered_list);
            }

            PrioritizeActions::Remove { index } => {
                let _ = main_list.remove(index.parse::<usize>().unwrap());
                let ordered_list = main_list.formatted_ordered_items(0, false);
                print!("\n\n{}\n", ordered_list);
            }
            PrioritizeActions::Done { index } => {
                let _ = main_list.toggle_done(index.parse::<usize>().unwrap());
                let ordered_list = main_list.formatted_ordered_items(0, false);
                print!("\n\n{}\n", ordered_list);
            }
            PrioritizeActions::List { pred, incomplete } => {
                #[cfg(debug_assertions)]
                let start = Instant::now();

                let pred = pred.unwrap_or(0u32);
                let ordered_list = main_list.formatted_ordered_items(pred, *incomplete);
                println!("\n\n{}\n", ordered_list);

                #[cfg(debug_assertions)]
                println!("listing took {:?}", start.elapsed());
            }

            PrioritizeActions::Carry { pred } => {
                println!("Moving previous days incomplete items to today...");
                let pred = pred.unwrap_or(1u32);
                let _ = main_list.carry_over_prev(pred);
                let ordered_list = main_list.formatted_ordered_items(0, false);
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
