use std::collections::HashSet;

fn main() {
    println!("Hello, world!");
}

#[derive(Debug)]
struct Task {
    id: u32,
    name: String,
    description: String,
    tags: HashSet<String>,
}
