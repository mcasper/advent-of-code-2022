use std::collections::{HashMap, VecDeque};

use anyhow::Result;
use regex::Regex;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 5, part 2 result: {}", result);
    Ok(())
}

#[derive(Debug)]
struct Instruction {
    quantity: i64,
    from: i64,
    to: i64,
}

#[derive(Debug)]
struct Stack {
    containers: VecDeque<char>,
}

impl Stack {
    fn push_front(&mut self, letter: char) {
        self.containers.push_front(letter);
    }

    fn push_back(&mut self, letter: char) {
        self.containers.push_back(letter);
    }

    fn pop(&mut self) -> Option<char> {
        self.containers.pop_front()
    }

    fn front(&self) -> Option<char> {
        self.containers.front().map(|c| c.clone())
    }
}

fn parse_instruction(line: String) -> Result<Instruction> {
    let instruction_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    let captures = instruction_re.captures(&line).unwrap();

    if captures.len() != 4 {
        return Err(anyhow::anyhow!(
            "couldn't capture all 3 for line '{}', got {}",
            line,
            captures.len()
        ));
    }

    return Ok(Instruction {
        quantity: captures
            .get(1)
            .map(|c| c.as_str())
            .unwrap()
            .parse::<i64>()?,
        from: captures
            .get(2)
            .map(|c| c.as_str())
            .unwrap()
            .parse::<i64>()?,
        to: captures
            .get(3)
            .map(|c| c.as_str())
            .unwrap()
            .parse::<i64>()?,
    });
}

fn parse_stacks(lines: Vec<String>) -> Result<HashMap<usize, Stack>> {
    let mut stack_map: HashMap<usize, Stack> = HashMap::new();

    for line in lines {
        let easier = " ".to_string() + &line;
        let chunks: Vec<String> = easier
            .as_bytes()
            .chunks(4)
            .map(|c| String::from_utf8(c.to_vec()).unwrap())
            .collect::<Vec<String>>();

        for (i, chunk) in chunks.iter().enumerate() {
            let index = i + 1;
            let letter = chunk.chars().nth(2).unwrap();
            if letter != ' ' {
                match stack_map.get_mut(&index) {
                    Some(stack) => {
                        stack.push_back(letter);
                    }
                    None => {
                        let s = Stack {
                            containers: VecDeque::from([letter]),
                        };

                        stack_map.insert(index, s);
                    }
                }
            }
        }
    }

    return Ok(stack_map);
}

fn run_instruction(stacks: &mut HashMap<usize, Stack>, instruction: Instruction) {
    let mut tmp_stack: Vec<Option<char>> = vec![];
    for _ in 0..instruction.quantity {
        let letter = stacks.get_mut(&(instruction.from as usize)).unwrap().pop();
        tmp_stack.push(letter);
    }
    tmp_stack.reverse();

    for letter in tmp_stack {
        if letter.is_some() {
            stacks
                .get_mut(&(instruction.to as usize))
                .unwrap()
                .push_front(letter.unwrap());
        }
    }
}

fn top_crates(stacks: HashMap<usize, Stack>) -> String {
    let mut keys: Vec<&usize> = stacks.keys().collect();
    keys.sort();
    let mut result = String::new();
    for key in keys {
        let top = stacks.get(key).unwrap().front().unwrap_or(' ');
        result.extend([top]);
    }
    result
}

fn solve(lines: Vec<String>) -> Result<String> {
    let mut stack_lines: Vec<String> = vec![];
    let mut parsing_stacks = true;
    let mut instructions: Vec<Instruction> = vec![];
    for line in lines {
        if line.starts_with(" 1") {
            parsing_stacks = false;
            continue;
        }

        if line.is_empty() {
            continue;
        }

        if parsing_stacks {
            stack_lines.push(line.clone());
        } else {
            instructions.push(parse_instruction(line.clone())?);
        }
    }
    let mut stacks = parse_stacks(stack_lines)?;

    for instruction in instructions {
        run_instruction(&mut stacks, instruction);
    }

    Ok(top_crates(stacks))
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
        let expected = "MCD";
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
