use std::fmt;
use std::fs;
use std::error::Error;

mod parser;

pub enum Command {
    List,
    Add(String),
    Mark(usize),
}

impl Command {
    pub fn parse_args(
        args: &mut impl Iterator<Item = String>,
    ) -> Result<Vec<Command>, &'static str> {
        let mut result = Vec::new();

        while let Some(curr) = args.next() {
            match curr.as_str() {
                "--list" | "-l" => result.push(Command::List),

                "--add" | "-a" => {
                    if let Some(add) = args.next() {
                        result.push(Command::Add(add));
                    } else {
                        return Err("Missing argument for add");
                    }
                }

                "--mark" | "-m" => {
                    if let Some(id) = args.next() {
                        let Ok(id) = id.parse::<usize>() else {
                            return Err("Id was not a number");
                        };

                        result.push(Command::Mark(id));
                    }
                }
                _ => return Err("Unknown Command"),
            }
        }
        Ok(result)
    }
}

pub struct TodoItem {
    id: usize,
    name: String,
    status: bool,
}

impl fmt::Display for TodoItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.status { 'x' } else { ' ' };

        write!(f, "[{}] {}", status, self.name)
    }
}

impl TodoItem {
    pub fn new(name: String, id: usize) -> TodoItem {
        TodoItem {
            id,
            name,
            status: false,
        }
    }

    pub fn build(name: String, id: usize, status: bool) -> TodoItem {
        TodoItem { id, name, status }
    }
}

pub struct TodoContainer(Vec<TodoItem>);

impl fmt::Display for TodoContainer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for item in &self.0 {
            write!(f, " - {}\n", item)?
        }
        Ok(())
    }
}

impl TodoContainer {
    pub fn list(&self) -> Vec<String> {
        let mut result = Vec::new();
        for item in &self.0 {
            result.push(format!("- {}", item));
        }
        result
    }

    pub fn add(&mut self, name: String) {
        self.0.push(TodoItem::new(name, self.0.len()));
    }

    pub fn mark(&mut self, id: usize) {
        for item in &mut self.0 {
            if item.id == id {
                item.status = true;
            }
        }
    }

    fn add_todo(&mut self, item: TodoItem) {
        self.0.push(item);
    }

    fn next_id(&self) -> usize {
        self.0.len()
    }

    pub fn new() -> TodoContainer {
        TodoContainer(Vec::new())
    }

    pub fn load() -> Result<TodoContainer, Box<dyn Error>> {
        let Ok(buffer) = fs::read_to_string("list.md") else {
            return Ok(TodoContainer::new());
        };

        let mut result = TodoContainer::new();
        for line in buffer.lines() {
            let Ok((name, status)) = parser::parse_line(&line) else {
                return Err("Malformed line in file".into());
            };

            result.add_todo(TodoItem::build(name, result.next_id(), status));
        }
        Ok(result)

    }

    pub fn save(&self) {
        let _ = fs::write("list.md", self.to_string());
    }

}

