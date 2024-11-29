use std::env;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("No file specified");
        return;
    }
    if args.len() > 2 {
        println!("To many parameters");
        return;
    }

    let path = Path::new(args[1].as_str());

    let mut file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(), why),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(why) => panic!("couldn't read {}: {}", path.display(), why),
        Ok(_) => print!("{}", s),
    }
}
