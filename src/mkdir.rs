use std::fs::create_dir_all;
use std::io;

pub fn mkdir(pwd: &String, args: Vec<&str>) {

    for i in 1..args.len() {

        let a = args[i];

        let path = [pwd, a].join("/");

        match create_dir_all(path) {
            Ok(_)   => {},
            Err(e)  => println!("Error: {}", e)
        }
    }

}