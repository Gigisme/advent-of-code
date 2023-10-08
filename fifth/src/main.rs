use std::path::Path;
use utils::read_lines;

fn main() {
    let file = Path::new("fifth/input");
    let lines = match read_lines(file) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    let mut unparsed_boxes: Vec<String> = Vec::new();
    for line_result in lines {
        let line = line_result.unwrap();
        if line.trim().is_empty() {
            break;
        }
        unparsed_boxes.push(line);
    }

    parse_boxes(unparsed_boxes);
}

fn parse_boxes(unparsed_boxes: Vec<String>) {
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
    for vec in all_boxes {
        println!("{:?}", vec)
    }
}


