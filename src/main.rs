use std::env;
use std::process;

use todolist::{Command, TodoContainer};

fn main() {
    let mut args = env::args();
    let mut list = TodoContainer::load().unwrap();
    args.next();
    let r = Command::parse_args(&mut args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    if r.len() == 0 {
        let _ = list.list().into_iter().map(|s| println!("{s}")).collect::<Vec<_>>();
    }

    for item in r {
        match item {
            Command::List => list.list().into_iter().map(|s| println!("{s}")).collect(),

            Command::Add(i) => list.add(i),

            Command::Mark(id) => list.mark(id),
        }
    }

    list.save();
}
