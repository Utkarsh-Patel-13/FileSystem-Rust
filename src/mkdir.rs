use std::fs::create_dir_all;

pub fn mkdir(pwd: &String, args: Vec<&str>) {

    for i in 1..args.len() {

        let a = args[i];

        // Set path for new directory from pwd.
        let path = [pwd, a].join("/");

        // Recursively create a directory and all of its parent components if they are missing.
        match create_dir_all(path) {
            Ok(_)   => {},
            Err(e)  => println!("Error: {}", e)
        }
    }

}