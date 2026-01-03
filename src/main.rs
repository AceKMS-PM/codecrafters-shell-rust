#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;

//use std::fs;
use pathsearch::find_executable_in_path;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
 loop {

    
     print!("$ ");
     io::stdout().flush().unwrap();
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
    let command_line = command.trim();

    const BUILTIN_COMMANDS: [&str; 3] = ["echo", "exit", "type"];
    //Liste dossier 
    //let path_env = env::var("PATH").unwrap();

    //// split(':') transforme "/bin:/usr/bin" en ["/bin", "/usr/bin"]
    //let paths = path_env.split(":");

    // On liste ici toutes les commandes "intégrées" (built-ins)


    if command_line.is_empty(){continue};



    let command_part : Vec<&str> = command_line.split_whitespace().collect();

    let command = command_part[0];

    let lenght = command_part.len();


    match command {
        "exit" => {
            if lenght > 1 {
                println!("one argument required");
                continue
            } else{break}
            
        }
        "echo" => {
            let message = command_part[1..].join(" ");
            println!("{}", message);
        }
        "type" => {
            if lenght < 2 {
                println!("type : missing argument");
            } else {
            if lenght < 2 {
                println!("type : missing argument");
            } else {
                let command_check = command_part[1];
            if BUILTIN_COMMANDS.contains(&command_check) {
                    println!("{} is a shell builtin", command_check);
                } else if let Some(path) = find_executable_in_path(command_check) {
                    println!("{} is {}", command_check, path.to_str().unwrap().to_string());
                } else {
                    println!("{}: not found", command_check);
                }
                
        };
                
        };
        }
        // Le "_" veut dire "tout le reste" (l'erreur)
        _ => {
            if let Some(_path) = find_executable_in_path(command){
                let status = Command::new(command)
                .args(&command_part[1..])
                .spawn()
                .and_then(| mut child| child.wait());
                
                if let Err(_) = status {
                    println!("{}: execution failed", command );
                }

            }else {
                println!("{}: command not found", command);
            }
        }

    }

    }
}
