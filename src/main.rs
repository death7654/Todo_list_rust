use std::fs;
use std::io;
use std::process::Output;
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
    let mut all_files = vector_files();
    let mut file = all_files[0].clone();
    while EXTENSION != "infinite_loop"
    {
        let mut command = String::from("");
        let _= (io::stdin().read_line(&mut command));
        command = command.to_string().clone();
        let split_command: Vec<&str> = command.split(" ").map(|x| x).collect::<Vec<&str>>();

        //all commands
        match split_command[0]
        {
            "add" => {}
            "remove"=> {}
            "edit" => {}
            "complete" => {}
            "change_list" => {file = all_files[split_command[1].parse::<usize>().unwrap()].clone()}
            "create_list" => {create_list((split_command[1..]).to_vec());}
            "delete_list" => {delete_list(split_command[1].parse().unwrap());}
            "edit_list" => edit_list(split_command[1].parse().unwrap(), &file),
            "lists"=> list_files(),
            "quit"=> break,
            _ => {println!("Invalid Command")}
        }
        all_files = vector_files();

    }
    println!("Hello, world!");
}
fn create_list(vector: Vec<&str>)
{
    let binding = vector.join(" ");
    let name = binding.as_str();
    let _ = fs::File::create_new("C:\\todo".to_owned() + name + EXTENSION);
}
fn delete_list(index: usize)
{
    let files = vector_files();
    let output = fs::remove_file("C:\\todo".to_owned()+ &files[index]);
    match output {
        Ok(_) => println!("{}, has been successfully deleted", &files[index]),
        Err(e) => println!("The file has not been deleted \n\n{}", e)
    }
}

fn edit_list(index: usize, name: &str)
{
    let files = vector_files();
    let original = &files[index.clone()];
    let output = fs::rename(original, name.to_owned() + EXTENSION);
    match  output {
        Ok(_) => println!("The list has been renamed successfully"),
        Err(e) => println!("An Error has occured \n\n{}",e)
    }

}

fn list_files()
{
    for file in fs::read_dir("C:\\todo").unwrap()
    {
        println!("{}", file.unwrap().path().display());
    }
}

fn vector_files() -> Vec<String>
{
    let mut vector = vec![];
    for file in fs::read_dir("C:\\todo").unwrap()
    {
        let _ = vector.push(file.unwrap().path().display().to_string());
    }
    vector
}
