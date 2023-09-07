use std::collections::HashSet;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, Lines};
use std::path::Path;

fn main() {
    let file_name = Path::new("third/input");
    let lines = match read_lines(file_name) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    let score:Vec<i32> = part_one(lines);
    let sum:i32 = score.iter().sum();
    println!("{}", sum);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn find_first_repeat(first_half: &str, second_half: &str) -> Option<char> {
    let mut chars = HashSet::new();
    for char in first_half.chars() {
        chars.insert(char);
    }
    for char in second_half.chars() {
        if chars.contains(&char) {
            return Some(char);
        }
    }
    return None;
}

fn score_letter(letter: char) -> u8 {
    return if letter.is_lowercase() {
        letter as u8 - 96
    } else {
        letter as u8 - 38
    }
}

fn part_one(lines: Lines<BufReader<File>>) -> Vec<i32> {
    let mut scores: Vec<i32> = Vec::new();
    for line in lines {
        match line {
            Ok(string) => {
                let half = string.split_at(string.len()/2);
                let repeat = find_first_repeat(half.0, half.1);
                match repeat {
                    None => {}
                    Some(letter) => {
                        scores.push(score_letter(letter) as i32);
                    }
                }
            }
            Err(e) => panic!("Problem reading: {}", e)
        }
    }
    scores
}