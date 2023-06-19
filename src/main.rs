use std::env;
use std::process;

use todolist::Command;

fn main() {
    let mut args = env::args();
    args.next();
    let r = Command::parse_args(&mut args)
        .unwrap_or_else(|err| {
            eprintln!("{err}");
            process::exit(1);
        });

    for item in r {
        match item {
            Command::List => todolist::list(),

            Command::Add(i) => todolist::add(&i),
        }
    }
}




