use std::env;
use std::env::Args;
use std::process;
use std::error::Error;

fn main() {
    let mut args = env::args();
    args.next();
    match args.next() {
        Some(command) => {
            run(command, &mut args)
            .unwrap_or_else(|err| {
                    eprintln!("Somthing went wrong: {err}");
                    process::exit(1);
                })
        },
        None => {
            eprintln!("No command given");
            process::exit(1);
        } 
    }
}

enum Command {
    List,
    Add(String),
}

impl Command {
    fn from(s: &str, args: &mut Args) -> Result<Command, &'static str> {
        match s {
            "--list" | "-l" => Ok(Command::List),
            "--add" => {
                if let Some(item) = args.next() {
                    Ok(Command::Add(item))
                } else {
                    Err("Missing Argument for add")
                }
            }
            _ => Err("Unknown command"),
        }
    }
}

fn run(c: String, args: &mut Args) -> Result<(), Box<dyn Error>>{
    let command = Command::from(&c, args)?;
    match command {
        Command::List => list(),
        Command::Add(i) => add(&i),
    }

    Ok(())
}

fn list() {
    println!("Test");
}

fn add(item: &String) {
    println!("{item}");
}

