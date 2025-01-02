use std::fs;
use std::io;
const EXTENSION: &str = ".txt";

const HELP: &str = "Usage: todo [COMMAND] [ARGUMENTS]
Todo is a super fast and simple tasks organizer written in rust
Example: todo list
Available commands:
    - add [TASK/s]
        adds new task/s
        Example: todo add \"buy carrots\"
    - edit [INDEX] [EDITED TASK/s]
        edits an existing task/s
        Example: todo edit 1 banana
    - list
        lists all tasks
        Example: todo list
    - done [INDEX]
        marks task as done
        Example: todo done 2 3 (marks second and third tasks as completed)
    - rm [INDEX]
        removes a task
        Example: todo rm 4
    - reset
        deletes all tasks
    - restore 
        restore recent backup after reset
    - sort
        sorts completed and uncompleted tasks
        Example: todo sort
    - raw [todo/done]
        prints nothing but done/incompleted tasks in plain text, useful for scripting
        Example: todo raw done
";



fn main() {
    let create_directory = fs::create_dir_all("/todo");
    match create_directory
    {
        Ok(_) => {},
        Err(e) => println!("Unable to create directory \n\n {}", e)
    }
    list_files();
    
    println!("The program is running.\nType :quit to quit");
    while EXTENSION != "infinite_loop"
    {
        let mut command = String::from("");
        let _ = io::stdin().read_line(&mut command);
        if command.contains(":quit")
        {
            break;
        }
    }
    println!("Hello, world!");
}

fn list_files()
{
    for file in fs::read_dir("C:\\todo").unwrap()
    {
        println!("{}", file.unwrap().path().display());
    }
}
