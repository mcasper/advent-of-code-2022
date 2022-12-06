use anyhow::Result;
use std::collections::HashSet;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 4, part 2 result: {}", result);
    Ok(())
}

fn get_range(s: String) -> Result<std::ops::RangeInclusive<i64>> {
    let bounds: Vec<String> = s.split("-").map(|s| s.into()).collect();
    if bounds.len() != 2 {
        return Err(anyhow::anyhow!("not 2"));
    }

    let start = bounds.first().unwrap().parse::<i64>()?;
    let end = bounds.last().unwrap().parse::<i64>()?;
    Ok(std::ops::RangeInclusive::new(start, end))
}

fn overlap(range1: std::ops::RangeInclusive<i64>, range2: std::ops::RangeInclusive<i64>) -> bool {
    let range1_set: HashSet<i64> = HashSet::from_iter(range1.into_iter());
    let range2_set: HashSet<i64> = HashSet::from_iter(range2.into_iter());

    let intersection: Vec<&i64> = range1_set.intersection(&range2_set).collect();
    intersection.len() > 0
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let mut overlaps = 0;

    for line in lines {
        let pairs: Vec<String> = line.split(",").map(|s| s.into()).collect();
        if pairs.len() != 2 {
            panic!("not 2")
        }

        let range1 = get_range(pairs[0].clone())?;
        let range2 = get_range(pairs[1].clone())?;

        if overlap(range1.clone(), range2.clone()) {
            overlaps += 1;
        }
    }

    Ok(overlaps)
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
        let expected = 4;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
