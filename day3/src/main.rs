use regex::Regex;
use std::{fs::File, io::Read, process::exit};

#[allow(dead_code)]
fn part1(buffer: String) -> i32 {
    let re = Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let commands: Vec<_> = re.find_iter(&buffer).map(|m| m.as_str()).collect();
    let comm_string: Vec<String> = commands.iter().map(|&x| x.to_string()).collect();
    let mut sum: i32 = 0;
    for mut command in comm_string {
        command.pop();
        for _ in 0..4 {
            command.remove(0);
        }
        let (val1, val2) = command.split_once(",").unwrap();
        //println!("(val1: {val1}, val2: {val2})");
        let val1: i32 = val1.parse().unwrap();
        let val2: i32 = val2.parse().unwrap();
        sum += val1 * val2
    }
    sum
}
#[allow(dead_code)]
fn part2(buffer: String) -> i32 {
    let mut sum = 0;
    let mut enable = true; //enables reading mul command
    let re = Regex::new(r"(?P<mul>mul\((?P<val1>[0-9]{1,3}),(?P<val2>[0-9]{1,3})\))|(?P<do>do\(\))|(?P<dn>don't\(\))"
    )
        .unwrap();
    for capture in re.captures_iter(&buffer) {
        if capture.name("do").is_some() {
            enable = true;
        }
        if capture.name("dn").is_some() {
            //println!("dn");
            enable = false;
        }
        if capture.name("mul").is_some() {
            //println!("enable: {enable}");
            if enable {
                let val1: i32 = capture["val1"].parse().unwrap();
                let val2: i32 = capture["val2"].parse().unwrap();
                sum += val1 * val2;
            }
        }
    }
    sum
}

fn main() {
    let path = "input.txt";
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            println!("unable to open a file: {path}, error: {e}");
            exit(1)
        }
    };
    let mut buffer = String::new();
    if let Err(e) = file.read_to_string(&mut buffer) {
        println!("Error while reading a file: {path}, error: {e:?}");
        exit(2);
    }

    //println!("sum: {}",part1(buffer))
    println!("sum: {}", part2(buffer))
}
