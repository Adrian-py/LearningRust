use std::io;
use std::io::Write;

enum Command {
    View,
    Add,
    Remove,
    Exit,
    Unknown,
}

impl Command {
    fn from(input: u8) -> Command {
        match input {
            1 => Command::View,
            2 => Command::Add,
            3 => Command::Remove,
            4 => Command:: Exit,
            _ => Command:: Unknown, 
        }
    }
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
    let command_input: Command = match command_input.trim().parse() {
        Ok(com) => {
            Command::from(com)
        },
        Err(_) => {
            println!("Please enter a command number!");
            Command::Unknown
        }
    };
    command_input
}

fn view_tasks(task_list: &Vec<String>) {
    println!("===================================");
    println!("| No. | Task Name                 |");
    println!("===================================");

    if task_list.is_empty() {
        println!("|     | {0: <26}|", "");
    } else {
        for (index, task) in task_list.iter().enumerate() {
            println!("|  {0: <2} | {1: <26}|", index + 1, task);
        }
    }
    println!("===================================");
}

fn add_task(task_list: &mut Vec<String>) {
    let mut new_task_name = String::new();
    loop {
        print!("Task Name: ");
        let _ = io::stdout().flush();
        io::stdin()
            .read_line(&mut new_task_name)
            .expect("Failed to read line!");
        new_task_name = new_task_name.trim().to_string();
        if new_task_name.len() > 0 {
            break;
        } else {
            println!("Can't be empty!");
        }
    }
    task_list.push(new_task_name.trim().to_string());
    println!("Update: Successfully added new task!");
}

fn remove_task(task_list: &mut Vec<String>) {
    if task_list.is_empty() {
        println!("You don't have any tasks at the moment");
        return;
    }

    let mut remove_task_index = String::new();
    view_tasks(task_list);
    println!("Which task would you like to remove?");
    print!(">> ");
    let _ = io::stdout().flush();
    io::stdin()
        .read_line(&mut remove_task_index)
        .expect("Failed to read line!");

    match remove_task_index.trim().parse::<usize>() {
        Ok(ind) => {
            if ind > task_list.len() || ind < 1 {
                println!("Taks not found!");
                return;
            }
            task_list.remove(ind - 1);
            println!("Update: Successfully removed task!");
        },
        Err(_) => {
            println!("Please enter a task number!");
        }
    };
}

fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        println!("\nTodo List App");
        println!("==============");

        let command: Command = handle_command_input();
        match command {
            Command::View => {
                view_tasks(&task_list);
            },
            Command::Add => {
                add_task(&mut task_list);
            },
            Command::Remove => {
                remove_task(&mut task_list);
            },
            Command::Exit => {
                break;
            },
            Command::Unknown => {
                println!("Unknown command!");
            },
        };
        
        println!("Press enter to continue...");
        let mut buffer = String::new();
        io::stdin()
            .read_line(&mut buffer)
            .expect("Failed to read line!");
        println!("\n\n\n\n");
    };
}
