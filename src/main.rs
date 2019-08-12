use std::env as env;
use std::fs::File;
use std::io::Read;

fn main() {
    let a: Vec<String>;
    let b: Vec<String>;
    let mut tmp = String::new();
    match env::args().len() {
        2 => {
            let mut file = match File::open(env::args().last().unwrap()) {
                Ok(f) => f,
                Err(_) => panic!("Can't open file"),
            };
            file.read_to_string(&mut tmp).unwrap();
            a = tmp.split("\n\n").collect::<Vec<&str>>()[0]
                .split("\n").map(|v| String::from(v)).collect::<Vec<String>>().clone();
            b = tmp.split("\n\n").collect::<Vec<&str>>()[1]
                .split("\n").map(|v| String::from(v)).collect::<Vec<String>>().clone();
        },
        3 => {
            let mut f1 = match File::open(env::args().nth(1).unwrap()) {
                Ok(f) => f,
                Err(_) => panic!("Can't open file 1"),
            };
            let mut f2 = match File::open(env::args().nth(2).unwrap()) {
                Ok(f) => f,
                Err(_) => panic!("Can't open file 2"),
            };
            f1.read_to_string(&mut tmp).unwrap();
            a = tmp.split("\n").map(|v| String::from(v)).collect::<Vec<String>>().clone();
            tmp.clear();
            f2.read_to_string(&mut tmp).unwrap();
            b = tmp.split("\n").map(|v| String::from(v)).collect::<Vec<String>>().clone();
        },
        _ => {
            panic!("Expected 1 or 2 files!")
        }
    }
    println!("A:\n{:?}\nB:\n{:?}",
             a.iter().filter(|v1| !b.iter().any(|v2| *v1 == v2)).collect::<Vec<_>>(),
             b.iter().filter(|v1| !a.iter().any(|v2| *v1 == v2)).collect::<Vec<_>>());
}
