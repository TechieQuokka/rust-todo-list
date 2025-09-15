use models::todo::{Todo, Priority};
use std::thread::sleep;
use std::time::Duration;

mod models {
    pub mod todo;
}

fn main () {

    println!("test start!");

    let mut todo = Todo::new("Rust Study!".to_string());
    println!("Todo : {:?}", todo);

    sleep(Duration::from_secs(2));

    todo.complete();
    println!("Todo : {:?}", todo);

    todo.update_title("Rust Wow!!!".to_string());
    println!("Todo : {:?}", todo);

    println!("priority : {:?}", todo.priority);
}