use std::io;
use std::vec::Vec;

fn main() {
    print!("Enter the amount of items you want in your list: ");
    let mut num_str: String = String::new();
    let mut thing: String = String::new();
    io::stdin()
        .read_line(&mut num_str)
        .expect("Failed to read line.");

    let num: i32 = num_str.parse().expect("Unable to parse");

    let mut vec: Vec<&str> = Vec::new();

    for _ in 0..num {
        io::stdin()
            .read_line(&mut thing)
            .expect("Failed to read line.");

        vec.push(&thing);

        // IDK WHAT IS GOING ON HERE
    }

    for x in vec {
        println!("{:?}", x)
    }
}
