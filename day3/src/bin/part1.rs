use anyhow::Result;
use std::collections::HashMap;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines);
    println!("Day 3, part 1 result: {}", result);
    Ok(())
}

fn priority(c: char) -> i64 {
    // a..z is 97-122
    // A-Z is 65-90
    // Abuse ASCII codes to get priority

    let ascii = c as u8;
    // println!("ascii: {}", ascii);
    if ascii >= 65 && ascii <= 90 {
        return (ascii - 38) as i64;
    }

    if ascii >= 97 && ascii <= 122 {
        return (ascii - 96) as i64;
    }

    unreachable!();
}

fn solve(lines: Vec<String>) -> i64 {
    let mut miscategorized_items: Vec<char> = vec![];

    'line: for line in lines {
        // println!("line: {}", line);
        let mut seen = HashMap::new();
        let (first_compartment, second_compartment) = line.split_at(line.len() / 2);

        for letter in first_compartment.chars() {
            seen.insert(letter, 1);
        }

        for letter in second_compartment.chars() {
            if seen.contains_key(&letter) {
                miscategorized_items.push(letter);
                continue 'line;
            }
        }
    }

    let mut result = 0;
    for item in miscategorized_items {
        // println!(
        //     "Miscategorized: {:?}, priority is {:?}",
        //     item,
        //     priority(item)
        // );
        result += priority(item);
    }
    result
}

fn lines(path: String) -> Result<Vec<String>> {
    let input_data: String = String::from_utf8(std::fs::read(path)?)?;
    let l: Vec<String> = input_data
        .trim()
        .split('\n')
        .map(|input| input.trim().to_string())
        .filter(|input| input != "")
        .collect();
    Ok(l)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let expected = 157;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}
