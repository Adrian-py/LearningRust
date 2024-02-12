use std::io;
use std::io::Write;

fn handle_command_input() -> u8 {
    println!("1. View all tasks");
    println!("2. Add new task");
    println!("3. Exit");
    print!(">> ");
    let _ = io::stdout().flush();
    
    let mut command_input = String::new();
    io::stdin().read_line(&mut command_input).expect("Failed to read line");
    let command_input: u8 = command_input.trim().parse().expect("Not a number!");

    command_input
}

fn view_tasks(task_list: &Vec<String>) {
    println!("===================================");
    println!("| No. | Task Name                 |");
    println!("===================================");

    if task_list.len() == 0 {
        println!("|     | {0: <26}|", "");
    } else {
        for (index, task) in task_list.iter().enumerate() {
            println!("|  {0: <2} | {1: <26}|", index + 1, task);
        }
    }

    println!("===================================");
    println!("Press enter to continue...");

    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).expect("Failed to read line!");
    println!("\n\n");
}

fn add_task(task_list: &mut Vec<String>) {
    let mut new_task_name = String::new();
    loop {
        print!("Task Name: ");
        let _ = io::stdout().flush();
        io::stdin().read_line(&mut new_task_name).expect("Failed to read line!");
        new_task_name = new_task_name.trim().to_string();
        if new_task_name.len() > 0 {
            break;
        } else {
            println!("Can't be empty!");
        }
    }
    let new_task_name = new_task_name.trim().to_string();
    task_list.push(new_task_name);
}

fn main() {
    let mut task_list: Vec<String> = Vec::new();
    loop {
        println!("\nTodo List App");
        println!("==============");
        let command: u8 = handle_command_input();
        match command {
            1 => {
                view_tasks(&task_list);
            },
            2 => {
                add_task(&mut task_list);
            },
            3 => {
                break;
            },
            _ => {}
        };
    }
}
