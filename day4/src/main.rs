use std::{
    fs,
    io::{BufRead, BufReader}
};

fn read_input(path: &str) -> Result<Vec<String>, std::io::Error> {
    let file = fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut lines: Vec<String> = Vec::new();
    for line in reader.lines() {
        lines.push(line?);
    }
    Ok(lines)
}

fn part1(input: &[String]) -> usize {
    let mut sum = 0;
    for row in input {
        sum += row.matches("XMAS").count();
        sum += row.matches("SAMX").count();
    }
    let rows = input.len();
    let cols = input[0].len();
    let matrix: Vec<&[u8]> = input.iter().map(|s| s.as_bytes()).collect();
    //column check
    for c in 0..cols {
        let mut column_match = String::new();
        for r in matrix.iter() {
            column_match.push(r[c] as char);
        }
        sum += column_match.matches("XMAS").count();
        sum += column_match.matches("SAMX").count();
    }
    //diagonal check top-left -> top-right
    for r in 0..rows {
        let mut diagonal_match = String::new();
        let mut i = r;
        let mut j = 0;
        while i < rows && j < cols {
            diagonal_match.push(matrix[i][j] as char);
            i += 1;
            j += 1;
        }
        sum += diagonal_match.matches("XMAS").count();
        sum += diagonal_match.matches("SAMX").count();
    }
    for c in 1..cols {
        let mut diagonal_match = String::new();
        let mut i = 0;
        let mut j = c;
        while i < rows && j < cols {
            diagonal_match.push(matrix[i][j] as char);
            i += 1;
            j += 1;
        }
        sum += diagonal_match.matches("XMAS").count();
        sum += diagonal_match.matches("SAMX").count();
    }
    //diagonal check bottom-left -> top-right
    for r in 0..rows {
        let mut diagonal_match = String::new();
        let mut i = r;
        let mut j = cols - 1;
        while i < rows {
            diagonal_match.push(matrix[i][j] as char);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        sum += diagonal_match.matches("XMAS").count();
        sum += diagonal_match.matches("SAMX").count();
    }
    for c in (1..cols).rev() {
        let mut diagonal_match = String::new();
        let mut i = 0;
        let mut j = c;
        while i < rows && j < cols - 1 {
            diagonal_match.push(matrix[i][j] as char);
            i += 1;
            if j == 0 {
                break;
            }
            j -= 1;
        }
        sum += diagonal_match.matches("XMAS").count();
        sum += diagonal_match.matches("SAMX").count();
    }

    sum
}

fn x_check(matrix: &[[char; 3]; 3]) -> usize {
    let mut diag1 = String::new(); //diag top-left -> bottom-right
    let mut diag2 = String::new(); //diag bottom-left -> top-right
    for (idx,element) in matrix.iter().enumerate() {
        diag1.push(element[idx]);
        diag2.push(element[2 - idx]);
    }
    if (diag1=="MAS"||diag1=="SAM")&&(diag2=="MAS"||diag2=="SAM"){
        1
    }else {
        0
    }
}

fn part2(input: &[String]) -> usize {
    let mut sum = 0usize;
    let rows = input.len();
    let cols = input[0].len();
    let mut char_matrix: [[char; 3]; 3]=Default::default(); //matrix for checking X-MAS

    for r in 0..rows - 2 {
        for c in 0..cols - 2 {
            for i in 0..3{
                for j in 0..3{
                    char_matrix[i][j]=input[r+i].as_bytes()[c+j] as char;
                }
            }
            sum += x_check(&char_matrix);
        }
    }

    sum
}

fn main() {
    let file_path="input.txt";
    // let file_path = "test.txt";
    let input = match read_input(file_path) {
        Ok(lines) => lines,
        Err(e) => panic!("Error reading file: {}", e),
    };
    // println!("result: {}",part1(&input))
    println!("result: {}", part2(&input));
}
