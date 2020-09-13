use std::fs;
use std::path;

pub fn ls(pwd: &String) {

    let p = path::Path::new(pwd);
    let entries: Vec<_> = fs::read_dir(p).unwrap().collect();

    for entry in entries {

        match entry {
            Ok(f) => {
                
                if let Some(file_name) = f.file_name().to_str() {

                    if let Ok(metadata) = f.metadata() {
                        if metadata.is_file() {
                            println!("{}", file_name)
                        }
                        else if metadata.is_dir() {
                            println!("{}/", file_name)
                        }
                    }
                }
            },
            Err(_) => {}
        }
    }   
}