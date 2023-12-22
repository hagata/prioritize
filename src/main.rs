use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use serde_json;
use std::error::Error;
use std::fs::OpenOptions;
use std::path::Path;

use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::io::{stdout, Result};
use std::path::PathBuf;

use clap::{Parser, Subcommand};

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

#[derive(Debug, Serialize, Deserialize, Default)]
struct List {
    todos: Vec<ListItem>,
}

impl List {
    pub fn new() -> Self {
        Self { todos: Vec::new() }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct ListItem {
    task: String,
    priority: Option<u32>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut main_list = List::new();
    // Load existing items from the file
    let path = Path::new("todos.json");
    if path.exists() {
        let file = fs::File::open(path)?;
        main_list = serde_json::from_reader(file)?;
    }

    // matches just as you would the top level cmd
    match &cli.command {
        Some(Commands::Prioritize { action }) => match action {
            PrioritizeActions::Add { task, priority } => {
                // todo!();
                // let priority = Some(priority);
                main_list.todos.push(ListItem {
                    task: task.clone(),
                    priority: *priority,
                });

                // Serialize the updated list and write it to the file
                let json = serde_json::to_string_pretty(&main_list)?;
                fs::write("todos.json", json)?;
                // println!("Adding task: '{}' with priority: {}", task, priority);
                dbg!(main_list);
            }
            PrioritizeActions::List {} => {
                dbg!(&main_list);
            }
        },
        // start the UI when no commands are passed
        None => {
            println!("No command ");
            stdout().execute(EnterAlternateScreen)?;
            enable_raw_mode()?;

            let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
            terminal.clear()?;

            loop {
                terminal.draw(|frame| {
                    let area = frame.size();
                    frame.render_widget(
                        Paragraph::new("Hello Ratatui! (press 'q' to quit)")
                            .white()
                            .on_dark_gray(),
                        area,
                    );
                })?;

                if event::poll(std::time::Duration::from_millis(16))? {
                    if let event::Event::Key(key) = event::read()? {
                        if key.kind == KeyEventKind::Press {
                            match key.code {
                                KeyCode::Char('Q') | KeyCode::Char('q') => {
                                    // Handle the case when 'Q' or 'q' is pressed
                                    break;
                                }
                                // Add more cases for other key codes if needed
                                _ => {
                                    // Handle other key codes when pressed
                                }
                            }
                        }
                    }
                } // TODO handle events
            }

            stdout().execute(LeaveAlternateScreen)?;
            disable_raw_mode()?;
        }
    }

    Ok(())

    // Continued program logic goes here...
}
