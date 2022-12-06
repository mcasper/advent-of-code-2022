use std::collections::{HashMap, VecDeque};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 5, part 1 result: {}", result);
    Ok(())
}

struct Buffer {
    inner: VecDeque<char>,
}

impl Buffer {
    fn add(&mut self, c: char) {
        if self.inner.len() >= 4 {
            self.inner.pop_front();
        }

        self.inner.push_back(c);
    }

    fn at_marker(&self) -> bool {
        if self.inner.len() != 4 {
            return false;
        }

        let mut seen = HashMap::new();
        for c in &self.inner {
            if let Some(_) = seen.get(&c) {
                return false;
            }

            seen.insert(c, true);
        }

        true
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let line = lines.first().unwrap();
    let mut b = Buffer {
        inner: VecDeque::new(),
    };
    for (i, c) in line.chars().enumerate() {
        b.add(c);
        if b.at_marker() {
            return Ok((i + 1) as i64);
        }
    }

    Err(anyhow::anyhow!("didn't find the marker"))
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
        let expected = 7;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
