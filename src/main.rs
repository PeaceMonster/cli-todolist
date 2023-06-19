use std::env;
use std::process;

use todolist::{ Command, TodoContainer};

fn main() {
    let mut args = env::args();
    let mut list = TodoContainer::new();
    args.next();
    let r = Command::parse_args(&mut args)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    for item in r {
        match item {
            Command::List => list.list()
                .into_iter()
                .map(|s| println!("{s}"))
                .collect(),

            Command::Add(i) => list.add(i),
        }
    }
}




