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
    let mut badges: Vec<char> = vec![];

    'chunk: for chunk in lines.chunks(3) {
        if chunk.len() != 3 {
            panic!("uh oh")
        }

        let mut group_seen: HashMap<char, u32> = HashMap::new();

        for line in chunk {
            let mut line_seen = HashMap::new();

            for letter in line.chars() {
                if line_seen.contains_key(&letter) {
                    continue;
                }
                line_seen.insert(letter, 1);

                if let Some(v) = group_seen.get(&letter) {
                    if *v == 2 {
                        badges.push(letter);
                        continue 'chunk;
                    } else {
                        group_seen.insert(letter, *v + 1);
                    }
                } else {
                    group_seen.insert(letter, 1);
                }
            }
        }
    }

    let mut result = 0;
    for badge in badges {
        // println!("Badge: {:?}, priority is {:?}", badge, priority(badge));
        result += priority(badge);
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
        let expected = 70;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap());
        assert_eq!(expected, actual);
    }
}
