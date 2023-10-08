use std::fs::File;
use std::io::{BufReader, Lines};
use std::num::ParseIntError;
use std::path::Path;
use utils::read_lines;

fn main() {
    let file_name = Path::new("fourth/input");
    let lines = read_lines(file_name);
    let mut result: i32 = 0;
    match lines {
        Ok(lines) => {
            result = task(lines);
        }
        Err(e) => {panic!("Error reading file, {}", e)}
    }
    println!("{}", result);
}

fn pare_to_i32_vec(input: Vec<&str>) -> Result<Vec<i32>, ParseIntError>
{
    input.iter().map(|s| s.parse()).collect()
}

fn parse_string(line: String) -> Option<(Vec<i32>, Vec<i32>)> {
    let halves: Vec<&str> = line.split(',').collect();
    let first: Vec<&str> = halves[0].split('-').collect();
    let second: Vec<&str> = halves[1].split('-').collect();

    let first_as_i32: Result<Vec<i32>, _> = pare_to_i32_vec(first);
    let second_as_i32: Result<Vec<i32>, _> = pare_to_i32_vec(second);

    if first_as_i32.is_ok() && second_as_i32.is_ok() {
        return Some((first_as_i32.unwrap(), second_as_i32.unwrap()))
    }
    return None
}

fn has_overlap(left: Vec<i32>, right: Vec<i32>) -> bool {
    //println!("{:?}, {:?}", left, right);
    if left[0] <= right[0] && left[1] >= right[1]
    { return true }
    if left[0] >= right[0] && left[1] <= right[1]
    { return true }
    return false
}

fn has_overlap_2(range1: Vec<i32>, range2: Vec<i32>) -> bool {
    let end_of_range1 = range1[range1.len() -1];
    let end_of_range2 = range2[range2.len() - 1];

    (range1[0] <= end_of_range2 && end_of_range1 >= range2[0]) ||
        (range2[0] <= end_of_range1 && end_of_range2 >= range1[0])
}

fn task(lines: Lines<BufReader<File>>) -> i32 {
    let mut score: i32 = 0;
    for line in lines {
        match line {
            Ok(line) => {
                let parsed = parse_string(line);
                match parsed {
                    None => {eprintln!("Error parsing")}
                    Some(result) => {
                        // Change this part for part one
                        if has_overlap_2(result.0, result.1)
                        {score += 1}
                    }
                }
            }
            Err(e) => {panic!("Error reading line, {}", e)}
        }
    }
    return score
}