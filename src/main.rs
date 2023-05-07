use std::env;
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;
use std::fs::OpenOptions;

fn main() {
    let args: Vec<String> = env::args().collect();
    let path = Path::new("C:\\ProgramData\\Noted\\notes.ntd");
    let display = path.display();

    if args[1] == "l" {
        let mut file = match File::open(&path) {
            Err(_) => match File::create(path) {
                Err(why) => panic!("Couldn't create file {}: {}", display, why),
                Ok(file) => file,
            }
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(why) => panic!("Couldn't read {}: {}", display, why),
            Ok(_) => (),
        }
        for (idx, line) in s.lines().enumerate() {
            println!("{}.\t{}", idx + 1, line);
        }
    } else {
        let mut file = match OpenOptions::new().write(true).read(true).open(path) {
            Err(_) => match File::create(path) {
                Err(why) => panic!("Couldn't create file {}: {}", display, why),
                Ok(file) => file,
            }
            Ok(file) => file,
        };

        let mut s = String::new();
        match file.read_to_string(&mut s) {
            Err(_) => s = String::from(""),
            Ok(_) => (),
        }

        file = File::create(path).unwrap();

        if args[1] == "r" {
            if args.len() > 2 {
                s = s.lines().take(args[2].parse::<usize>().unwrap() - 1)
                    .map(|line| String::from(line) + "\n").collect::<Vec<String>>().concat()
                    + &s.lines().skip(args[2].parse::<usize>().unwrap())
                    .map(|line| String::from(line) + "\n").collect::<Vec<String>>().concat();
                file.write_all(s.as_bytes()).unwrap()
            }
        } else {
            s.push_str(&format!("{}\n", &args[1..].join(" ")));
            //println!("{}", s);
            match file.write_all(s.as_bytes()) {
                Err(why) => panic!("Couldn't write to {}: {}", display, why),
                Ok(_) => ()
            }
        }
    }
}
