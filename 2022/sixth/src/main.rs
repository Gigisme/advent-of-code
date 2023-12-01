use std::path::Path;
use utils::read_lines;

fn main() {
    let file = Path::new("2022/sixth/input");
    let mut lines = match read_lines(file) {
        Ok(lines) => lines,
        Err(e) => panic!("Problem opening file: {}", e)
    };
    let line = lines.next().unwrap().unwrap();
    // println!("{}", find_marker_index(line, 4)) // part one
    println!("{}", find_marker_index(line, 14))

}

fn find_marker_index(data: String, marker_length: usize) -> usize {
    let mut pre_marker: Vec<char> = Vec::new();
    let mut index: usize = 0;
    for char in data.chars() {
        if pre_marker.len() == marker_length
        {
            if unique_chars(&pre_marker)
            {break;}
            pre_marker.remove(0);
        }
        pre_marker.push(char);
        index += 1;
    }
    return index;
}

fn unique_chars(data: &Vec<char>) -> bool {
    for i in 0..data.len()
    {
        for j in i+1..data.len() {
            if data[i] == data[j]
            { return false; }
        }
    }
    return true
}