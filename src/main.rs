#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // TODO: Uncomment the code below to pass the first stage
 loop {
     print!("$ ");
     io::stdout().flush().unwrap();
     let mut command = String::new();
     io::stdin().read_line(&mut command).unwrap();
    let command_line = command.trim();

    if command_line.is_empty(){continue};

    let command_part : Vec<&str> = command_line.split_whitespace().collect();

    let command = command_part[0];


     if command == "exit" { // if command == "exit\n"
        break
     }
     else if command == "echo" {
// On prend tous les mots APRES le premier et on les recolle avec un espace
        let message = &command_part[1..].join(" ");
        println!("{}", message);
        continue
     } 
     else
     {     
        println!("{}: command not found", command.trim());
        };

 }
}
