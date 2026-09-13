use std::io::{self, Write};
use std::vec::Vec;

fn main() {
    let mut vec: Vec<String> = Vec::new();

    println!("-----WELCOME-----");
    println!("Use 'help' to get a list of commands!");

    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush().expect("Unable to flush stdout");

        io::stdin()
            .read_line(&mut input)
            .expect("Unable to read line. Please try again...");

        let input = input.to_lowercase();
        match input.trim() {
            // Add item
            "add" => {
                print!("Enter item to add: ");
                io::stdout().flush().expect("Unable to flush stdout");
                let mut item = String::new();

                io::stdin()
                    .read_line(&mut item)
                    .expect("Unable to read item.");

                vec.push(item.clone());

                println!("Added {} to list!", item.trim());
            }
            // Remove item
            "remove" => {
                print!("Enter item to remove: ");
                io::stdout().flush().expect("Unable to flush stdout");
                let mut item = String::new();

                io::stdin()
                    .read_line(&mut item)
                    .expect("Unable to read item.");

                for i in 0..vec.len() {
                    if vec[i] == item {
                        vec.remove(i);
                        break;
                    }
                }
                println!("Removed {} from the list.", item.trim());
            }
            "view" => {
                for i in 0..vec.len() {
                    print!("{}. {}", i + 1, vec[i]);
                }
            }
            "help" => println!("add, remove, view, help, exit"),
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}
