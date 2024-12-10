use std::{
    fs::File,
    io::{BufRead, BufReader},
    process::exit,
};

fn read_file(path: &str) -> (Vec<i32>, Vec<i32>) {
    let file = match File::open(path) {
        Ok(file) => file,
        Err(e) => {
            print!("unable to open {} file, err: {}", path, e);
            exit(1)
        }
    };
    //file reading
    let buffer = BufReader::new(file);
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    //let lines = buffer.lines(); unnecessary
    //splitting to 2 diffrent list
    for line in buffer.lines() {
        // first version
        // let line = line.unwrap();
        // let values:Vec<&str> =line.split("   ").collect();
        // list1.push(values[0].parse().unwrap());
        // list2.push(values[1].parse().unwrap());
        //better version
        match line {
            Ok(line_content) => {
                // Parse and collect values into separate lists
                if let Some((first, second)) = line_content.split_once("   ") {
                    list1.push(first.parse::<i32>().unwrap());
                    list2.push(second.parse::<i32>().unwrap());
                }
            }
            Err(e) => {
                eprintln!("Error reading line: {}", e);
                exit(1);
            }
        }
    }
    (list1, list2)
}

fn part1(mut list1: Vec<i32>, mut list2: Vec<i32>) -> i32 {
    //sorting list
    // list1.sort();
    // list2.sort();
    // faster sort
    list1.sort_unstable();
    list2.sort_unstable();

    // let mut distances:Vec<i32> = Vec::new();
    // for idx in 0..list1.len(){
    //     distances.push((list1[idx]-list2[idx]).abs())
    // }

    // let sum:i32 = distances.into_iter().sum();

    //better aproach
    list1
        .iter()
        .zip(list2.iter())
        .map(|(x1, x2)| (x1 - x2).abs())
        .sum()
}

fn part2(mut list1: Vec<i32>, mut list2: Vec<i32>)->i32{
    list1.iter().map(|&x| x*list2.iter().filter(|&&y| y==x).count() as i32).sum()
}
fn main() {
    //file opening
    let path = "input.txt";
    let (list1, list2) = read_file(path);

    //println!("sum of distances: {}", part1(list1, list2))
    println!("similarity score: {}", part2(list1, list2))
}
