use std::path::Path;
use regex::Regex;
use utils::read_lines;

fn main() {
    let file = Path::new("fifth/input");
    let mut lines = match read_lines(file) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    let mut unparsed_boxes: Vec<String> = Vec::new();
    let mut reading_boxes = true;
    let mut boxes = vec![];
    for line_result in lines {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            reading_boxes = false;
            let owned_vec = std::mem::replace(&mut unparsed_boxes, Vec::new());
            boxes = parse_boxes(owned_vec);
            continue;
        }
        if reading_boxes {
            unparsed_boxes.push(line);
        }
        else {
            let commands = parse_command(&line);
            //boxes = move_boxes_p1(boxes, commands[0], commands[1] as usize, commands[2] as usize)
            boxes = move_boxes_p2(boxes, commands[0], commands[1] as usize, commands[2] as usize)
        }
    }
    for row in boxes {
        println!("{:?}", row)
    }
}

fn parse_boxes(unparsed_boxes: Vec<String>) -> Vec<Vec<char>> {
    let mut all_boxes: Vec<Vec<char>> = Vec::new();
    for _ in 0..9 {
        let v:Vec<char> = Vec::new();
        all_boxes.push(v);
    }
    for string in &mut unparsed_boxes.iter().rev().skip(1) {
        let line = string.replace("]"," ").replace("["," ");
        for i in (0..line.len() - 2).step_by(4)
        {
            let char = line.chars().nth(i+1).unwrap();
            if char != ' ' {
                all_boxes[i/4].push(char);
            }
        }
    }
    return all_boxes;
}

fn parse_command(string: &str) -> Vec<i32> {
    let mut numbers: Vec<i32> = Vec::new();
    let re = Regex::new(r"\d+").unwrap();
    for capture in re.captures_iter(string) {
        let number = capture.get(0).unwrap().as_str();
        numbers.push(number.parse::<i32>().unwrap());
    }
    return numbers;
}

fn move_boxes_p1(mut boxes: Vec<Vec<char>>, amount: i32, from_index: usize, to_index:usize) -> Vec<Vec<char>> {
    for _ in 0..amount {
        let mv_box = boxes[from_index - 1].pop().unwrap();
        boxes[to_index - 1].push(mv_box);
    }
    return boxes;
}

fn move_boxes_p2(mut boxes: Vec<Vec<char>>, amount: i32, from_index: usize, to_index: usize) -> Vec<Vec<char>> {
    let mut mv_box: Vec<char> = Vec::new();
    for _ in 0..amount {
        mv_box.push(boxes[from_index - 1].pop().unwrap())
    }
    for _ in 0..amount {
        boxes[to_index - 1].push(mv_box.pop().unwrap())
    }
    return boxes;
}