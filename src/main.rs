use clap::{App,Arg,Command};

#[derive(Parser, Debug)]
#[command(author,version,about,long_about=None)]
struct Args {
    #[arg(short, long)]
    task: String,

    #[arg(short, long, default_value_t = 3)]
    priority: u8,

    // display commands
    #[arg(short,long)]
    list: 
}

fn main() {
let matches = App::new("PrioritizeCLI")
    .version("0.0.1")
    .author("Genki")
    .about("Todolist app for one day at a time")
    .subcommand(
        App::new("add")
            .about("Add a new todo task")
            .arg(Arg::with_name("task").index(1).required(true).help("Task title"))
            .arg(Arg::with_name("priority").short("p").long("priority").takes_value(true).help("Task priority")),
    )
    .get_matches();

    match matches.subcommand() {
    ("add", Some(sub_m)) => {
        let title = sub_m.value_of("title").unwrap();
        let priority = sub_m.value_of("priority").unwrap_or("default_priority");

        // Now, you can execute the "add" operation with the provided title and priority.
        // Handle the actual logic here.
    }
    _ => {
        // Handle other subcommands or display help/error messages as needed.
    }
}

}
