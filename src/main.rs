mod dialogMenu;
mod commands;
mod parse;
mod image;

use std::io::{self, Write};

fn main(){
    let choose: i8 = loop{
        println!("Choose option: ");
        println!("1) Open the file");
        println!("2) Exit\n");
        print!("Your choice: ");

        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("ERROR: Failed to read input");

        match input.trim().parse::<i8>(){
            Ok(num) if num >= 1 && num <= 2 => break num,
            Ok(_) => println!("ERROR: the number must be at least 1 and at most 2!"),
            Err(_) => println!("ERROR: only number!"),
        }
    };

    println!("");

    let mut path: Option<String> = None;
    match choose{
        1=> {
            path = dialogMenu::pick_image_file()
        }
        _ => println!("Exiting..."),
    }

    if let Some(file_path) = path {
        match parse::parse_bmp_file(file_path) {
            Ok(bmp) => {
                println!("Width: {}px", bmp.width);
                println!("Height: {}px", bmp.height);
                println!("Binary size in bytes: {}", bmp.data.len());
            }
            Err(e) => println!("ERROR: {}", e),
        }
    }

    commands::pause_terminal();
}