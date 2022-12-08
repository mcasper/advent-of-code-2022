use std::{collections::HashMap, path::PathBuf};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 7, part 2 result: {}", result);
    Ok(())
}
enum ParserState {
    SearchingForCommand,
    ParsingCommandOutput,
}

#[derive(Debug)]
struct Directory {
    files: Vec<File>,
    directories: Vec<String>,
}

impl Directory {
    fn size(&self, tree: &HashMap<String, Directory>) -> usize {
        let mut result = 0;
        for file in &self.files {
            result += file.size;
        }

        for dirname in &self.directories {
            let other_dir = tree.get(dirname).unwrap();
            result += other_dir.size(&tree);
        }

        result
    }
}

#[derive(Debug)]
struct File {
    size: usize,
}

#[derive(Debug)]
struct State {
    pwd: PathBuf,
    tree: HashMap<String, Directory>,
}

impl State {
    fn cd(&mut self, dir: String) {
        match dir.as_ref() {
            ".." => {
                self.pwd.pop();
            }
            "/" => self.pwd = PathBuf::from("/"),
            _ => self.pwd = self.pwd.join(dir),
        }
    }

    fn add_entry(&mut self, line: String) {
        let split: Vec<&str> = line.split(" ").collect();
        if split[0] == "dir" {
            self.add_dir(split[1]);
        } else {
            self.add_file(split[0].parse::<usize>().unwrap());
        }
    }

    fn add_dir(&mut self, dirname: &str) {
        let current_path = self.pwd.to_str().unwrap();
        let full_dir_path = self.pwd.join(dirname);
        let full_dirname = full_dir_path.to_str().unwrap();

        if dirname != "/" {
            let parent = self.tree.get_mut(&current_path.to_string()).unwrap();
            parent.directories.push(full_dirname.to_string());
        }

        let dir = Directory {
            directories: vec![],
            files: vec![],
        };
        self.tree.insert(full_dirname.to_string(), dir);
    }

    fn add_file(&mut self, size: usize) {
        let dir = self.tree.get_mut(self.pwd.to_str().unwrap()).unwrap();
        dir.files.push(File { size: size })
    }
}

fn solve(lines: Vec<String>) -> Result<usize> {
    let mut parser_state = ParserState::SearchingForCommand;
    let mut current_state = State {
        pwd: PathBuf::from("/"),
        tree: HashMap::new(),
    };
    current_state.add_dir("/");

    for line in lines {
        match parser_state {
            ParserState::SearchingForCommand => {
                if !line.starts_with("$ ") {
                    // continue;
                    panic!("Searching for command, didn't find one")
                }

                let parts: Vec<&str> = line.split(" ").collect();
                match parts[1] {
                    "cd" => {
                        current_state.cd(parts[2].into());
                    }
                    "ls" => {
                        parser_state = ParserState::ParsingCommandOutput;
                    }
                    _ => {
                        panic!("Unknown command {}", parts[1]);
                    }
                }
            }
            ParserState::ParsingCommandOutput => {
                if line.starts_with("$ ") {
                    parser_state = ParserState::SearchingForCommand;
                    let parts: Vec<&str> = line.split(" ").collect();
                    match parts[1] {
                        "cd" => {
                            current_state.cd(parts[2].into());
                        }
                        "ls" => {
                            parser_state = ParserState::ParsingCommandOutput;
                        }
                        _ => {
                            panic!("Unknown command {}", parts[1]);
                        }
                    }
                    continue;
                }

                current_state.add_entry(line);
            }
        }
    }

    let total_space = 70000000;
    let min_unused_space = 30000000;

    let root_dir = current_state.tree.get("/").unwrap();
    let current_unused_space = total_space - root_dir.size(&current_state.tree);
    let min_free_size = min_unused_space - current_unused_space;

    let mut smallest_viable_free_size = total_space;
    for key in current_state.tree.keys() {
        let dir = current_state.tree.get(key).unwrap();
        let size = dir.size(&current_state.tree);

        if size > min_free_size && size < smallest_viable_free_size {
            smallest_viable_free_size = size;
        }
    }

    Ok(smallest_viable_free_size)
}

fn lines(path: String) -> Result<Vec<String>> {
    let input_data: String = String::from_utf8(std::fs::read(path)?)?;
    let l: Vec<String> = input_data
        .split('\n')
        .map(|input| input.to_string())
        .filter(|input| input != "")
        .collect();
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let expected = 24933642;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
