pub mod task_handler {
    use prettytable::{row, Table};
    use std::io;
    use std::io::Write;

    pub fn view_tasks(task_list: &Vec<String>) {
        println!("Your Tasks");
        let mut tasks_table = Table::new();
        tasks_table.set_titles(row!["No.", "Task Name"]);
        if task_list.is_empty() {
            tasks_table.add_row(row!["", ""]);
        } else {
            for (index, task) in task_list.iter().enumerate() {
                tasks_table.add_row(row![index + 1, task]);
            }
        }
        tasks_table.printstd();
    }

    pub fn add_task(task_list: &mut Vec<String>) {
        println!("Adding a New Task");
        loop {
            let mut new_task_name = String::new();

            print!("Task Name: ");
            let _ = io::stdout().flush();
            io::stdin()
                .read_line(&mut new_task_name)
                .expect("Failed to read line!");
            new_task_name = new_task_name.trim().to_string();

            if new_task_name.len() < 1 {
                println!("Can't be empty!");
                continue;
            }
            if new_task_name.len() >= 20 {
                println!("Length of task name must be between 1 - 20!");
                continue;
            }

            task_list.push(new_task_name.trim().to_string());
            break;
        }
        println!("Update: Successfully added new task!");
    }

    pub fn remove_task(task_list: &mut Vec<String>) {
        if task_list.is_empty() {
            println!("You don't have any tasks at the moment");
            return;
        }

        println!("Deleting a Task");
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
            }
            Err(_) => {
                println!("Please enter a task number!");
            }
        };
    }
}
