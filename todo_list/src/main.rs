#[macro_use]
extern crate prettytable;

use std::io;
use std::io::Write;

mod command;
use crate::command::Command;

mod tasks;
use crate::tasks::task_handler;

fn clear_screen() {
    // Clears screen and moves cursor to top left of terminal screen
    print!("\x1B[2J\x1B[1;1H");
}

fn handle_command_input() -> Command {
    println!("1. View all tasks");
    println!("2. Add new task");
    println!("3. Remove a task");
    println!("4. Exit");
    print!(">> ");
    let _ = io::stdout().flush();

    let mut command_input = String::new();
    io::stdin()
        .read_line(&mut command_input)
        .expect("Failed to read line");
    let command_input: Command = match command_input.trim().parse::<u8>() {
        Ok(com) => Command::from(com),
        Err(_) => {
            println!("Please enter a command number!");
            Command::Unknown
        }
    };

    command_input
}

fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        clear_screen();
        println!("Todo List App");
        println!("==============");

        let command: Command = handle_command_input();
        clear_screen();
        match command {
            Command::View => task_handler::view_tasks(&task_list),
            Command::Add => task_handler::add_task(&mut task_list),
            Command::Remove => task_handler::remove_task(&mut task_list),
            Command::Exit => break,
            Command::Unknown => println!("Unknown command!"),
        };

        println!("Press enter to continue...");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        println!("\n\n\n\n");
    }
}
