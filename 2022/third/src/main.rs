use std::collections::HashSet;
use std::fs::File;
use utils::read_lines;
use std::io::{BufReader, Lines};
use std::path::Path;

fn main() {
    let file_name = Path::new("2022/third/input");
    let lines = match read_lines(file_name) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    //let score:Vec<i32> = part_one(lines);
    //let sum:i32 = score.iter().sum();
    //println!("Part one result: {}", sum);
    let score:Vec<i32> = part_two(lines);
    let sum:i32 = score.iter().sum();
    println!("{}", sum);

}

fn find_first_repeat(first: &str, second: &str) -> Option<Vec<char>> {
    let mut chars = HashSet::new();
    let mut repeats: Vec<char> = Vec::new();
    for char in first.chars() {
        chars.insert(char);
    }
    for char in second.chars() {
        if chars.contains(&char) {
            repeats.push(char);
        }
    }
    if repeats.len() > 0
    {
        return Some(repeats)
    }
    return None
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
                    None => {panic!("No repeats found")}
                    Some(letter) => {
                        scores.push(score_letter(letter[0]) as i32);
                    }
                }
            }
            Err(e) => panic!("Problem reading: {}", e)
        }
    }
    scores
}

fn part_two(lines: Lines<BufReader<File>>) -> Vec<i32> {
    let mut scores: Vec<i32> = Vec::new();
    let mut chunk: Vec<String> = Vec::new();
    for line in lines {
        match line {
            Ok(string) => {
                chunk.push(string);
            }
            Err(e) => panic!("Problem reading: {}", e)
        }
        if chunk.len() == 3 {
            let repeats = find_first_repeat(&*chunk[0], &*chunk[1]);
            match repeats {
                None => {panic!("No repeats found")}
                Some(repeats) => {
                    let mut letters: String = String::new();
                    for char in repeats {
                        letters.push(char);
                    }
                    let badge = find_first_repeat(&*chunk[2],&letters);
                    match badge {
                        None => {panic!("No repeats found")}
                        Some(letter) => {
                            scores.push(score_letter(letter[0]) as i32)
                        }
                    }
                }
            }
            chunk.clear();
        }
    }
    scores
}