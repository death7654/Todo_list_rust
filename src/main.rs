use fancy::printcoln;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Read;
use std::path::PathBuf;
use std::str::FromStr;
use std::usize;
const EXTENSION: &str = ".txt";

#[derive(Clone)]
struct Todo {
    map: HashMap<String, bool>,
}
impl Todo {
    fn add(&mut self, key: String) {
        self.map.insert(key, false);
    }
    fn save(self, file: &str) {
        let mut content = String::new();
        for (key, value) in self.map {
            let record = format!("{}\t{}\n", key, value);
            content.push_str(&record)
        }
        let _ = std::fs::write(file, content);
    }
    fn new(file: &str) -> Result<Todo, std::io::Error> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file)?;
        let mut content = String::new();
        f.read_to_string(&mut content)?;
        let map: HashMap<String, bool> = content
            .lines()
            .map(|line| line.splitn(2, '\t').collect::<Vec<&str>>())
            .map(|v| (v[0], v[1]))
            .map(|(k, v)| (String::from(k), bool::from_str(v).unwrap()))
            .collect();
        Ok(Todo { map })
    }
}

fn main() {
    let create_directory = fs::create_dir_all("/todo");
    match create_directory {
        Ok(_) => {}
        Err(e) => println!("Unable to create directory \n\n {}", e),
    }
    //lists files
    let mut all_files = vector_files();
    if vector_files().len() == 0 {
        let path = PathBuf::from("C:\\todo\\default.txt");
        let _ = fs::File::create(path);
        all_files = vector_files();
    }
    list_files();

    //selecting file
    println!("\nChoose a file");
    let mut command = String::from("");
    let _ = io::stdin().read_line(&mut command);
    command = command.to_string().clone();
    let split: Vec<&str> = command.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();
    let file_index = split[0].parse::<usize>().unwrap() - 1;
    let mut file = all_files[file_index].clone();

    let mut todo_orig: Todo = Todo::new(&file).expect("failure");

    println!("\nThe program is running.\nType quit to quit\n");
    while EXTENSION != "infinite_loop" {
        let todo: Todo = todo_orig.clone();

        //getting commands
        command = String::from("");
        let _ = io::stdin().read_line(&mut command);
        command = command.to_string().clone();
        let split_command: Vec<&str> = command.split(" ").map(|x| x.trim()).collect::<Vec<&str>>();

        //all commands
        let request = split_command[0].to_owned();
        match request {
            _ if request.contains("add_task") => {
                let task = &split_command[1..].join(" ");
                todo_orig.add(task.to_string());
                todo_orig.clone().save(&file);
            }
            _ if request.contains("remove_task") => {
                todo_orig.map.remove(&split_command[1].to_string());
                todo_orig.clone().save(&file);
            }
            _ if request.contains("rename_task") => {
                todo_orig.map.remove(&split_command[1].to_string());
                todo_orig.add(split_command[2].to_string());
                todo_orig.clone().save(&file);
            }
            _ if request.contains("complete_task") => {
                todo_orig.map.insert(split_command[1].to_string(), true);
            }
            _ if request.contains("uncomplete_task") => {
                todo_orig.map.insert(split_command[1].to_string(), false);
            }
            _ if request.contains("list_tasks") => {
                let list = &todo.map;
                let mut num = 1;
                for (key, _) in list {
                    let key_value = list.get(key);
                    let convert = key_value.clone().unwrap().clone();
                    if convert == true {
                        printcoln!("[:strikethrough]{}. {}", num, key);
                    } else {
                        printcoln!("[:]{}. {}", num, key);
                    }
                    num += 1;
                }
            }
            _ if request.contains("change_list") => {
                let picked_file = all_files[split_command[1].parse::<usize>().unwrap() - 1].clone();
                if change_list(&picked_file) {
                    file = picked_file;
                }
                todo_orig = Todo::new(&file).expect("Failure")
            }
            _ if request.contains("create_list") => {
                create_list((split_command[1..]).to_vec());
            }
            _ if request.contains("delete_list") => {
                delete_list(split_command[1].parse().unwrap());
                file = all_files[0].clone();
            }
            _ if request.contains("rename_list") => {
                edit_list(split_command[1].parse().unwrap(), &split_command[2])
            }
            _ if request.contains("current_list") => println!("{file}"),
            _ if request.contains("lists") => list_files(),
            _ if request.contains("help") => help(),
            _ if request.contains("quit") => {
                todo_orig.save(&file);
                break;
            }
            _ => {
                println!("\nInvalid Command")
            }
        }
        all_files = vector_files();
    }
}

fn change_list(file: &String) -> bool {
    let path = PathBuf::from("C:\\todo\\").join(file);
    println!("{:#?}", path);
    let check = File::open(path);
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
    let name = binding.as_str().to_owned() + EXTENSION;
    let file = PathBuf::from("C:\\todo").join(name);
    let output = fs::File::create(file);
    match output {
        Ok(_) => println!("the file has been successfully created"),
        Err(e) => println!("The file has not been created \n\n{e}"),
    }
}
fn delete_list(index: usize) {
    let files = vector_files();
    let delete = &files[index];
    let output = fs::remove_file(delete);
    match output {
        Ok(_) => println!("{} has been successfully deleted", &files[index - 1]),
        Err(e) => println!("The file has not been deleted \n\n{e}"),
    }
}

fn edit_list(index: usize, name: &str) {
    let files = vector_files();
    let original = PathBuf::from("C:\\todo").join(&files[index - 1]);
    let mut new_name = "C:\\todo\\".to_owned();
    new_name.push_str(name);
    new_name.push_str(EXTENSION);
    let path = PathBuf::from(new_name);
    let output = fs::rename(original, path);
    match output {
        Ok(_) => println!("The list has been renamed successfully"),
        Err(e) => println!("An Error has occured \n\n{e}"),
    }
}

fn list_files() {
    let mut num = 1;
    for file in fs::read_dir("C:\\todo").unwrap() {
        println!("{num}. {}", file.unwrap().path().display());
        num += 1;
    }
}
fn help() {
    println!("Help Page\nadd_task [name]\n    - adds task\nremove_task [name]\n    - removes task\nrename_task [name]\n    - renames task\ncomplete_task [name]\n    - marks a task as complete\nuncomplete_task [name]\n    - makes a task a incomplete\nlist_tasks\n    - lists all tasks in the current list\nchange_list\n    - changes list\ncreate_list\n    - creates a list\ndelete_list\n    - deletes a list\nrename_list\n    - renames a list\ncurrent_list\n    - displays current list\nlists\n    - displays all lists\nhelp\n    - displays this page\nquit\n    - saves and quits this program");
}

fn vector_files() -> Vec<String> {
    let mut vector = vec![];
    for file in fs::read_dir("C:\\todo").unwrap() {
        let _ = vector.push(file.unwrap().path().display().to_string());
    }
    vector
}
