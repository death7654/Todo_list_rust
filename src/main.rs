use std::clone;
use std::fs;
use std::fs::File;
use std::io;
use fancy;
use std::process::Output;
use std::usize;
const EXTENSION: &str = ".txt";


struct file
{
    name: String,
    tasks: Vec<task>
}
struct task
{
    name: String,
    completed: bool
}

fn main() {
    let create_directory = fs::create_dir_all("/todo");
    match create_directory {
        Ok(_) => {}
        Err(e) => println!("Unable to create directory \n\n {}", e),
    }
    list_files();
    let mut all_files = vector_files();
    let mut file = all_files[0].clone();

    let mut selected = file{
        name: file.to_string(),
        tasks: Vec::new()
    };
    

    println!("The program is running.\nType quit to quit");
    while EXTENSION != "infinite_loop" {
        let mut command = String::from("");
        let _ = io::stdin().read_line(&mut command);
        command = command.to_string().clone();
        let split_command: Vec<&str> = command.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();

        //all commands
        let request = split_command[0].to_owned();
        match request {
            _ if request.contains("add") => {}
            _ if request.contains("remove") => {}
            _ if request.contains("edit") => {}
            _ if request.contains("complete") => {}
            _ if request.contains("change_list") => {
                let picked_file = all_files[split_command[1].parse::<usize>().unwrap()].clone();
                if change_list(&picked_file) {
                    file = picked_file;
                }
            }
            _ if request.contains("create_list") => {
                create_list((split_command[1..]).to_vec());
            }
            _ if request.contains("delete_list") => {
                delete_list(split_command[1].parse().unwrap());
            }
            _ if request.contains("edit_list") => {edit_list(split_command[1].parse().unwrap(), &split_command[2..])},
            _ if request.contains("lists") => list_files(),
            _ if request.contains("quit") => break,
            _ => {
                println!("Invalid Command")
            }
        }
        all_files = vector_files();
    }
    println!("Hello, world!");
}

fn change_list(file: &String) -> bool {
    let check = File::open("C:\\todo\\".to_owned() + &file + EXTENSION);
    match check {
        Ok(_) => return true,
        Err(e) => {
            println!("An Error has occured {e}");
            return false;
        }
    };
}
fn create_list(vector: Vec<&str>) {
    let binding = vector.join(" ");
    let name = binding.as_str();
    let _ = fs::File::create_new("C:\\todo".to_owned() + name + EXTENSION);
}
fn delete_list(index: usize) {
    let files = vector_files();
    let delete = &files[index];
    let output = fs::remove_file(delete);
    match output {
        Ok(_) => println!("{} has been successfully deleted", &files[index]),
        Err(e) => println!("The file has not been deleted \n\n{e}"),
    }
}

fn edit_list(index: usize, name: &[&str]) {
    println!("called");
    let files = vector_files();
    let original = &files[index.clone()];
    let string = name.join(" ");
    println!("{original}");
    let output = fs::rename(original, (string + EXTENSION));
    match output {
        Ok(_) => println!("The list has been renamed successfully"),
        Err(e) => println!("An Error has occured \n\n{e}"),
    }
}

fn list_files() {
    for file in fs::read_dir("C:\\todo").unwrap() {
        println!("{}", file.unwrap().path().display());
    }
}

fn vector_files() -> Vec<String> {
    let mut vector = vec![];
    for file in fs::read_dir("C:\\todo").unwrap() {
        let _ = vector.push(file.unwrap().path().display().to_string());
    }
    vector
}


fn json_to_struct(file: String)
{

}
