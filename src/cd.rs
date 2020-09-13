use std::fs;
use std::path;


// TODO: cd to inner directories, i.e cd d1/d2/d3
pub fn cd(pwd: &mut String, mut args: Vec<&str>) {
 
    // Remove all empty elements from args
    args.retain(|&x| x != "");
    
    // Can't have multiple directories as arguments. eg: cd dir1 dir2
    if args.len() > 2 {
        println!("> Too many args for cd command.");       
        return
    }
    
    // Jump to root directory
    else if args.len() == 1 || args[1].is_empty() {
        *pwd = String::from("./fs");
        return
    }

    // Jump to previous directory.
    else if args[1] == ".." && pwd != "./fs" {

        let mut temp: Vec<&str> = pwd.split("/").collect();

        temp.remove(temp.len() - 1);

        *pwd = temp.join("/");

        if pwd.ends_with("/") {
            pwd.remove(pwd.len() - 1);
        }
        return
    }

    // Jump to provided directory if it exists.
    else{

        let mut dir_name = String::from(args[1]);

        if dir_name.ends_with("/") {
            dir_name.remove(dir_name.len()-1);
        }

        let p = path::Path::new(pwd);
        let entries: Vec<_> = fs::read_dir(p).unwrap().collect();

        for entry in entries {

            match entry {
                Ok(f) => {
                    
                    if let Some(file_name) = f.file_name().to_str() {

                        if dir_name == file_name {
                            if let Ok(metadata) = f.metadata() {
                                if metadata.is_file() {
                                    println!("> Can't cd a file.");
                                    return
                                }
                                else if metadata.is_dir() {
                                    
                                    pwd.push_str("/");
                                    pwd.push_str(args[1]);

                                    if pwd.ends_with("/") {
                                        pwd.remove(pwd.len() - 1);
                                    }

                                    return
                                }
                            }
                        }
                    }
                },
                Err(_) => {}
            }
        }
    }
    println!("> No such directory.");
}