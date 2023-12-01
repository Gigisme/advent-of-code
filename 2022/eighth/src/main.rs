use std::fs::File;
use std::io::{BufReader, Lines};
use std::path::Path;
use utils::read_lines;

fn main() {
    let file_name = Path::new("2022/eighth/input");
    let lines = read_lines(file_name).unwrap();
    let data = pare_data(lines);
    // part_1(data);
    part_2(data);
}

fn pare_data(lines: Lines<BufReader<File>>) -> Vec<Vec<i8>> {
    let mut columns: Vec<Vec<i8>> = Vec::new();
    let mut vec_init = false;

    for line in lines {
        let line = line.unwrap();
        let mut chars = line.chars();
        let width = line.len();
        if !vec_init{
            (0..width).for_each(|_| columns.push(Vec::new()));
            vec_init=true;
        }
        for i in 0..width {
            if let Some(digit) = chars.next().unwrap().to_digit(10){
                columns[i].push(digit as i8);
            }
        }
    }
    return columns;
}

fn part_1(data: Vec<Vec<i8>>) {
    let width = data.len();
    let height = data[0].len();

    let mut up_max: Vec<i8> = Vec::new();
    let mut right_max: i8 = 0;
    let mut left_max: i8 = 0;
    let mut down_max: Vec<i8> = Vec::new();

    //outside lines ignored
    for i in 1..width-1 {
        up_max.push(data[i][0]);
        down_max.push(*data[i][2..height].iter().max().unwrap())
    }

    let mut visible_tress: i32 = (width * 2 + height * 2 - 4) as i32;

    for y in 1..height - 1 {
        left_max = data[0][y];
        let row: Vec<_> = data.iter().map(|inner_vec| inner_vec[y]).collect();
        right_max = *row[2..width].iter().max().unwrap();
        for x in 1..width - 1{
            let mut visible = false;
            let current_tree = data[x][y];

            if current_tree > left_max {
                visible = true;
                left_max = current_tree;
            }
            if current_tree > right_max {
                visible = true;
            }
            if current_tree == right_max {
                right_max = *row[(x + 1)..width].iter().max().unwrap();
                if current_tree > right_max { visible=true; }
            }
            if current_tree > up_max[x-1] {
                visible = true;
                up_max[x-1] = current_tree;
            }
            if current_tree > down_max[x-1] {
                visible = true;
            }
            if current_tree == down_max[x-1] {
                down_max[x-1] = *data[x][(y + 1)..height].iter().max().unwrap();
                if current_tree > down_max[x-1] { visible = true; }
            }

            if visible {
                visible_tress += 1;
            }
        }
    }
    println!("Visible tress: {}", visible_tress)
}

fn part_2(data: Vec<Vec<i8>>) {
    let width = data.len();
    let height = data[0].len();
    let mut max_score = 0;
    //edge score 0 so ignoring edges
    for y in 1..height - 1 {
        for x in 1..width-1 {
            let mut score = 0;
            score = check_up(&data, x, y);
            score *= check_down(&data, x, y);
            score *= check_left(&data, x, y);
            score *= check_right(&data, x, y);
            if score > max_score {
                max_score = score;
            }
        }

    }
    println!("Highest scenic score: {}", max_score);
}

fn check_up(data: &Vec<Vec<i8>>, x: usize, y: usize) -> i32 {
    let mut score: i32 = 0;
    let tree_size = data[x][y];

    let mut iterator = data[x][0..y].iter().rev();
    loop {
        if let Some(value) = iterator.next() {
            score += 1;
            if value >= &tree_size {
                break;
            }
        } else {break;}
    }
    return score;
}

fn check_down(data: &Vec<Vec<i8>>, x: usize, y: usize) -> i32  {
    let mut score = 0;
    let tree_size = data[x][y];

    for i in y+1..data[x].len() {
        score += 1;
        if data[x][i] >= tree_size {
            break;
        }
    }
    return score;
}

fn check_left(data: &Vec<Vec<i8>>, x: usize, y: usize) -> i32  {
    let mut score = 0;
    let tree_size = data[x][y];

    for i in (0..x).rev() {
        score +=1;
        if data[i][y] >= tree_size {
            break;
        }
    }
    return score;
}

fn check_right(data: &Vec<Vec<i8>>, x: usize, y: usize) -> i32  {
    let mut score = 0;
    let tree_size = data[x][y];

    for i in x+1..data.len() {
        score += 1;
        if data[i][y] >= tree_size {
            break;
        }
    }
    return score;
}