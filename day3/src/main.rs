use std::{fs::File, io::{BufReader, Read}, process::exit};

fn main() {
    let path="input.txt";
    let file=match File::open(path)  {
        Ok(file)=>file,
        Err(e)=> {println!("unable to open a file: {path}, error: {e}");
    exit(1)}
    };
    let buf=BufReader::new(file);

    


}
