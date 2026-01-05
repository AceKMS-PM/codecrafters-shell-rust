#[allow(unused_imports)]
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use std::env;

//use std::fs;
use pathsearch::find_executable_in_path;

fn main() {
   
 loop {

    
     print!("$ ");
     io::stdout().flush().unwrap();
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
    let command_line = command.trim();

    const BUILTIN_COMMANDS: [&str; 5] = ["echo", "exit", "type","pwd","cd"];
    //Liste dossier 
    //let path_env = env::var("PATH").unwrap();

    //// split(':') transforme "/bin:/usr/bin" en ["/bin", "/usr/bin"]
    //let paths = path_env.split(":");

    // On liste ici toutes les commandes "intégrées" (built-ins)

// Ligne de commande vide 
    if command_line.is_empty(){continue};


// segmenter l'entrée en paramtètrepar un espace
    let command_part : Vec<&str> = command_line.split_whitespace().collect();

// première valeurest la commande
    let command = command_part[0];

//Le nombre de paramètre
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
            }  else {
                let command_check = command_part[1]; // premier argument de type 
                if BUILTIN_COMMANDS.contains(&command_check) {
                        println!("{} is a shell builtin", command_check);
                    } else if let Some(path) = find_executable_in_path(command_check) { // verifier si command_check existent dand le Path et que ca retourne quelque chose
                        println!("{} is {}", command_check, path.to_str().unwrap().to_string()); // ffciher command et le path retourné si Oui 
                    } else {
                        println!("{}: not found", command_check);
                    }
                
                };     

        }
        "pwd" => {
            if lenght > 1 {
                println!("one argument required");
                continue;
            } else {
                match  env::current_dir() {
                    Ok(path) => println!("{}",path.display()), // Répertoire courant si OK 
                    Err(_) => println!("execution failed")
                }
            }
        }
        "cd" => {
            if lenght == 1 {
                continue;
            } else if lenght > 2 {
                println!("one argument required");
                continue;
            } else {
   
                let dir_att = command_part[1..].join(""); // arugment de cd 

                match dir_att.as_str()
                {           
                    "~" => {
                            if let Some(path) =  env::home_dir(){ // SI repertoire HOME est retourné 
                                match env::set_current_dir(path) { // Y aller dedans 
                                Ok(_good) => continue,
                                Err(_) => println!("Home directory not found")
                                }
                            }
                    }
                    _ => {
                            let path = Path::new(&dir_att);

                            match env::set_current_dir(path) { // Aller dans le repertoire path si existant
                                Ok(_good) => continue,
                                Err(_) => println!("cd: {}: No such file or directory",path.display())
                            }
                    }
                }
            }
        }
        // Le "_" veut dire "tout le reste" (l'erreur)
        _ => {
            if let Some(_path) = find_executable_in_path(command){ // verifier si command_check existent dand le Path et que ca retourne quelque chose
                let status = Command::new(command) //intialiser command 
                .args(&command_part[1..])   //récupérer l'arguemnt de la command
                .spawn()    // l'exécuter 
                .and_then(| mut child| child.wait());   // Dire au programme parent ( du shell ) d'attendre l'execution de cette command spawner
                
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
