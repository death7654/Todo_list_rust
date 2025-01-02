fn main() {
    let create_directory = fs::create_dir_all("/todo");
    match create_directory
    {
        Ok(_) => {},
        Err(e) => println!("Unable to create directory \n\n {}", e)
    }
    let files = fs::read_dir("C:\\todo");
    match files{
        Ok(i) =>{println!("Files {:#?}", i)},
        Err(_)=> {}
    }
    println!("files");

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

}
