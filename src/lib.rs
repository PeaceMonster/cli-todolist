use std::fmt;

pub enum Command {
    List,
    Add(String),
}

impl Command {
    pub fn parse_args(
        args: &mut impl Iterator<Item = String>
    ) -> Result<Vec<Command>, &'static str> {

        let mut result = Vec::new();

        while let Some(curr) = args.next() {

            let s = &curr[..];

            match s {
                "--list" | "-l"  => result.push(Command::List),

                "--add" | "-a" => {
                    if let Some(add) = args.next() {
                        result.push(Command::Add(add));
                    } else {
                        return Err("Missing argument for add");
                    }
                },

                 _ => return Err("Unknown Command"),
            }
        } 
        Ok(result)
    }
}

pub struct TodoItem {
    name: String,
    status: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.status {
            'x'
        } else {
            ' '
        };

        write!(f, "[{}] {}", status, self.name)
    }
}

impl TodoItem {
    pub fn new(name: String) -> TodoItem {
        TodoItem { name, status: false }
    }

    pub fn build(name: String, status: bool) -> TodoItem {
        TodoItem { name, status}
    }
}

pub struct TodoContainer(Vec<TodoItem>);

impl TodoContainer {
    pub fn list(&self) -> Vec<String> {
        let mut result = Vec::new();
        for item in &self.0 {
            result.push(
                format!("{}", item)
            );
        }
        result
    }
}


pub fn list() {
    println!("Test");
}

pub fn add(item: &String) {
    println!("{item}");
}
