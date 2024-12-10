use regex::Regex;
use std::{
    fs::File,
    io::{BufReader, Read},
    process::exit,
};

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
    let re =Regex::new(r"mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let commands:Vec<_>=re.find_iter(&buffer).map(|m| m.as_str()).collect();
    let comm_string:Vec<String>=commands.iter().map(|&x| {
       x.to_string()
    }).collect();
    let mut sum:i32=0;
    for mut command in comm_string{
        command.pop();
        for _ in 0..4{
            command.remove(0);
        }
        let (val1,val2)=command.split_once(",").unwrap();
        //println!("(val1: {val1}, val2: {val2})");
        let val1:i32=val1.parse().unwrap();
        let val2:i32=val2.parse().unwrap();
        sum+=val1*val2
    };
    println!("sum: {sum}")
    

}
