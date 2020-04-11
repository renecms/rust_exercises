use std::collections::HashMap;
use std::io;

enum Commands {
    ListDepartment(String),
    ListCompany,
    Add(String, String),
    Help,
    Quit,
    Invalid,
}

struct Database {
    departments: HashMap<String, Vec<String>>,
}
impl Database {
    fn new() -> Database {
        Database {
            departments: HashMap::new(),
        }
    }
    fn list_all(&self) {
        println!("Listing the whole company:");
        for department in self.departments.iter() {
            println!("\tDepartment: {}", department.0);
            for employee in department.1 {
                println!("\t\t{}", employee);
            }
            print!("\n");
        }
    }
    fn list_department(&self, department: &str) {
        if self.departments.get(department).is_some() {
            println!("Listing department {}:", department);
            for employee in self.departments.get(department).unwrap().iter() {
                println!("\t{}", employee);
            }
        } else {
            println!("Department {} does not exist", department);
        }
    }
    fn add_employee(&mut self, department: &str, employee: &str) {
        println!("Adding {} to {}", employee, department);
        self.departments
            .entry(department.to_string())
            .or_insert(Vec::new())
            .push(employee.to_string());
        println!("Employee added successfully");
    }
}

fn main() {
    let mut database = Database::new();

    println!("Please type your command. Type help for more information");

    loop {
        let mut command = String::new();

        io::stdin()
            .read_line(&mut command)
            .expect("Failed to read line");

        match get_command(&command).unwrap() {
            Commands::Quit => break,
            Commands::Help => print_help(),
            Commands::Add(department, employee) => database.add_employee(&department, &employee),
            Commands::ListCompany => database.list_all(),
            Commands::ListDepartment(department) => database.list_department(&department),
            Commands::Invalid => println!("Invalid command, type help for help"),
        }
    }
}

fn print_help() {
    println!("Commands available:");
    println!("\tadd <Employee> to <Department>");
    println!("\thelp");
    println!("\tlistall");
    println!("\tlist <Department>");
    println!("\tquit");
}

fn get_command(command_string: &String) -> Option<Commands> {
    let command = command_string.trim().to_lowercase();

    if command.starts_with("quit") {
        Some(Commands::Quit)
    } else if command.starts_with("add") {
        let split_command: Vec<&str> = command.split_whitespace().collect();
        if split_command.len() == 4 && split_command[2].to_lowercase() == "to" {
            Some(Commands::Add(
                split_command[3].to_string(),
                split_command[1].to_string(),
            ))
        } else {
            Some(Commands::Invalid)
        }
    } else if command.starts_with("listall") {
        Some(Commands::ListCompany)
    } else if command.starts_with("list") {
        let split_command: Vec<&str> = command.split_whitespace().collect();
        if split_command.len() == 2 {
            Some(Commands::ListDepartment(String::from(split_command[1])))
        } else {
            Some(Commands::Invalid)
        }
        
    } else if command.starts_with("help") {
        Some(Commands::Help)
    } else {
        Some(Commands::Invalid)
    }
}
