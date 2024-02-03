use std::{env, fs};

fn main() {
    let args:Vec<String> = env::args().collect();

    if args.len() != 3 || args[1] != "-c" {
        eprintln!("Usage: ccwc -c <filename>");
        std::process::exit(1);
    }

    let filename = &args[2];

    match fs::read(filename){
       Ok(contents) => println!("Number of bytes in {}: {}",filename,contents.len()),
        Err(err) => {
            eprintln!("Error reading file {}: {}",filename,err);
            std::process::exit(1);
        }
    }
}
