#[allow(unused_imports)]
use std::io::{self, Write};
use std::env;
//use std::fs;
use std::path::Path;

fn main() {
    // TODO: Uncomment the code below to pass the first stage
 loop {

    
     print!("$ ");
     io::stdout().flush().unwrap();
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
    let command_line = command.trim();

    //Liste dossier 
    let path_env = env::var("PATH").unwrap();

    //// split(':') transforme "/bin:/usr/bin" en ["/bin", "/usr/bin"]
    let paths = path_env.split(":");

    // On liste ici toutes les commandes "intégrées" (built-ins)


    if command_line.is_empty(){continue};



    let command_part : Vec<&str> = command_line.split_whitespace().collect();

    let command = command_part[0];

    let lenght = command_part.len();


    match command {
        "exit" => {
            break
        }
        "echo" => {
            let message = command_part[1..].join(" ");
            println!("{}", message);
        }
        "type" => {
            if lenght < 2 {
                println!("type : missing argument");
            } else {
            let command_check = command_part[1];
            let mut found = false;
            for directory in paths {
                    let full_path = std::path::Path::new(directory).join(command_check);
                    
                    if full_path.exists() {
                        println!("{} is {}", command_check, full_path.display());
                        found = true;
                        break;
                };

            };
            
            if !found
                {println!("{}: not found", command_check);}
                
        };
        }
        // Le "_" veut dire "tout le reste" (l'erreur)
        _ => {
            println!("{}: command not found", command);
        }

    }

    }
}
