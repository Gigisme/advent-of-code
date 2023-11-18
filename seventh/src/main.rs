use std::fs::File;
use std::io::{BufReader, Lines};
use std::path::Path;
use utils::read_lines;
use std::fmt;

const FILESYSTEM_SIZE: usize = 70000000;
const UPDATE_SIZE: usize = 30000000;

fn main() {
    let file_name = Path::new("seventh/input");
    let mut lines = read_lines(file_name).unwrap();
    // Skip first line, its root dir
    lines.next();
    let mut root : FileType = FileType::Directory { name: "/".to_string(), children: vec![] };
    parse_data(&mut root, lines);
    //part_1(root, 100000)
    part_2(root, UPDATE_SIZE)
}

enum FileType {
    File { name: String, size: usize},
    Directory { name: String, children: Vec<FileType>}
}

fn part_1(root: FileType, max_size: usize) {
    let files = root.to_vec();
    let mut result: usize = 0;
    for file in files {
        if let FileType::Directory {..} = file {
            let size = file.total_size();
            if size <= max_size {
                result += size
            }
        }
    }
    println!("{}", result)
}

fn part_2(root: FileType, required_space: usize) {
    let mut min = FILESYSTEM_SIZE;
    let mut result: usize = 0;

    let files = root.to_vec();
    let free_space = FILESYSTEM_SIZE - root.total_size();
    if free_space >= required_space {
        println!("Should not be possible")
    }
    let required_space = required_space - free_space;

    for file in files {
        if let FileType::Directory {..} = file {
            let size = file.total_size();
            if size >= required_space && size < min {
                min = size;
                result = file.total_size();
            }
        }
    }
    println!("{}", result)
}

fn parse_data(root: &mut FileType, mut lines: Lines<BufReader<File>>) -> Lines<BufReader<File>> {
    loop {
        // Default file
        let mut file: Option<FileType> = None;

        //Get line
        let line = match lines.next() {
            None => {return lines}
            Some(line) => {
                line.unwrap()
            }
        };
        let words: Vec<&str> = line.split(" ").collect();
        // Get to parsing
        match words[0] {
            "$" => {
                match words[1] {
                    "cd" => {
                        if words[2] == ".." { return lines }

                        if let FileType::Directory { children, .. } = root {
                            if let Some(found_file) = children.iter_mut().find(|file| {
                                if let FileType::Directory{ name, .. } = file {
                                    name == words[2]
                                } else {
                                    false
                                }
                            })
                            {
                                lines = parse_data(found_file, lines);
                            }
                            else {
                                println!("File not found: {}", words[2]);
                            }
                        }
                    }
                    "ls" => {}
                    _ => {
                        println!("Unknown command: {}", words[1]);
                        return lines;
                    }
                }
            }
            "dir" => {
                file = Some(FileType::Directory { name: words[1].to_string(), children: vec![] });
            }
            _ => {
                file = Some(FileType::File { name: words[1].to_string(), size: words[0].parse().unwrap() });
            }
        }
        match file{
            Some(file) => {
                if let FileType::Directory { children, .. } = root {
                    children.push(file);
                    // for child in children {
                    //     println!("{}", child)
                    // }
                }
            }
            None => {}
        }
    }
}

impl fmt::Display for FileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FileType::File { name, size } => write!(f, "File: {} (Size: {} bytes)", name, size),
            FileType::Directory { name, .. } => write!(f, "Directory: {}", name),
        }
    }
}

impl FileType {
    fn total_size(&self) -> usize {
        match self {
            FileType::File { size, .. } => *size,
            FileType::Directory {children, .. } => {
                children.into_iter().map(|f| f.total_size()).sum()
            }
        }
    }
    fn to_vec(&self) -> Vec<&FileType> {
        return match self {
            FileType::File { .. } => {
                vec![self]
            }
            FileType::Directory { children, .. } => {
                let mut files = vec![self];
                for child in children {
                    files.append(&mut child.to_vec());
                }
                files
            }
        }
    }
}