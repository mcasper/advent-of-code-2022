use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 11, part 1 result: {}", result);
    Ok(())
}

type MonkeyId = i64;

struct Monkey {
    id: MonkeyId,
    items: Vec<i64>,
    operation: Box<dyn Fn(i64) -> i64>,
    test: Box<dyn Fn(i64) -> MonkeyId>,
    inspected: i64,
}

impl From<&[String]> for Monkey {
    fn from(lines: &[String]) -> Self {
        let id_line_split = lines[0]
            .strip_prefix("Monkey ")
            .unwrap()
            .chars()
            .nth(0)
            .unwrap()
            .to_string()
            .parse::<i64>()
            .unwrap(); // DANGER - breaks for numbers larger than 9
        let id = id_line_split as MonkeyId;

        let items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<i64>().unwrap())
            .collect();

        let mut operation_tail = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .to_string();
        let operator = operation_tail.remove(0);
        let operation: Box<dyn Fn(i64) -> i64>;
        match operator {
            '+' => {
                let value = operation_tail.trim().parse::<i64>().unwrap();
                operation = Box::new(move |old: i64| -> i64 { old + value });
            }
            '*' => {
                if operation_tail.trim() == "old" {
                    operation = Box::new(move |old: i64| -> i64 { old * old });
                } else {
                    let value = operation_tail.trim().parse::<i64>().unwrap();
                    operation = Box::new(move |old: i64| -> i64 { old * value });
                }
            }
            _ => {
                unreachable!()
            }
        }

        let divisible_by = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let if_true = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let if_false = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<i64>()
            .unwrap();
        let test = Box::new(move |val: i64| -> MonkeyId {
            if val % divisible_by == 0 {
                if_true
            } else {
                if_false
            }
        });

        Monkey {
            id,
            items,
            operation,
            test,
            inspected: 0,
        }
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let mut monkies: Vec<Monkey> = lines.chunks(6).map(|c| c.into()).collect();
    monkies.sort_by_key(|m| m.id);

    for _ in 0..20 {
        for i in 0..monkies.len() {
            let items: Vec<i64> = monkies[i].items.drain(0..).collect();

            for item in items {
                let result = (monkies[i].operation)(item) / 3;
                let throw_to_id = (monkies[i].test)(result);

                if monkies[throw_to_id as usize].id != throw_to_id {
                    panic!("what?")
                }

                monkies[throw_to_id as usize].items.push(result);
                monkies[i].inspected += 1;
            }
        }
    }

    let mut inspected: Vec<i64> = monkies.iter().map(|m| m.inspected).collect();
    inspected.sort();
    inspected.reverse();

    Ok(inspected[0] * inspected[1])
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
        let expected = 10605;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
