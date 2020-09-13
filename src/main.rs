use std::io;
use std::io::{stdout,Write};
use std::fs::create_dir_all;

mod mkdir;
mod ls;

fn get_input() -> String {

    let _ = stdout().flush();

    let mut input = String::new();

    input.clear();
    match io::stdin().read_line(&mut input) {
        Ok(_)   => {},
        Err(e)  => println!("Error; {}", e)
    }
    input.trim().to_string();
    input.truncate(input.trim_end_matches(&['\n', '\r'][..]).len());

    input
}

fn main() {


    println!("Welcome  to FS!!\n");

    match create_dir_all("./fs") {
        Ok(_)   => {},
        Err(e)  => println!("Error: {}", e)
    }
   
    let mut pwd = String::from("./fs");

    
    let _ = stdout().flush();

    loop {
        
        let input = get_input();

        let input_arr: Vec<&str> = input.split(" ").collect();

        match input_arr[0] {

            "pwd"       => println!("> {}", pwd),

            "mkdir"     => mkdir::mkdir(&pwd, input_arr),

            "ls"        => ls::ls(&pwd),

            "exit"      => break,

            " " | "\n"  => continue, 

            _           => println!("=> Command {} not found.", input)

        }
    }
}
