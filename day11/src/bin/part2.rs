use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 11, part 2 result: {}", result);
    Ok(())
}

type MonkeyId = u64;

struct Monkey {
    id: MonkeyId,
    items: Vec<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    test: Box<dyn Fn(u64) -> MonkeyId>,
    inspected: u64,
    divisible_by: u64,
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
            .parse::<u64>()
            .unwrap(); // DANGER - breaks for numbers larger than 9
        let id = id_line_split as MonkeyId;

        let items = lines[1]
            .strip_prefix("  Starting items: ")
            .unwrap()
            .split(", ")
            .map(|s| s.parse::<u64>().unwrap())
            .collect();

        let mut operation_tail = lines[2]
            .strip_prefix("  Operation: new = old ")
            .unwrap()
            .to_string();
        let operator = operation_tail.remove(0);
        let operation: Box<dyn Fn(u64) -> u64>;
        match operator {
            '+' => {
                let value = operation_tail.trim().parse::<u64>().unwrap();
                operation = Box::new(move |old: u64| -> u64 { old + value });
            }
            '*' => {
                if operation_tail.trim() == "old" {
                    operation = Box::new(move |old: u64| -> u64 { old * old });
                } else {
                    let value = operation_tail.trim().parse::<u64>().unwrap();
                    operation = Box::new(move |old: u64| -> u64 { old * value });
                }
            }
            _ => {
                unreachable!()
            }
        }

        let divisible_by = lines[3]
            .strip_prefix("  Test: divisible by ")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let if_true = lines[4]
            .strip_prefix("    If true: throw to monkey ")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let if_false = lines[5]
            .strip_prefix("    If false: throw to monkey ")
            .unwrap()
            .parse::<u64>()
            .unwrap();
        let test = Box::new(move |val: u64| -> MonkeyId {
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
            divisible_by,
        }
    }
}

fn solve(lines: Vec<String>) -> Result<u64> {
    let mut monkeys: Vec<Monkey> = lines.chunks(6).map(|c| c.into()).collect();
    monkeys.sort_by_key(|m| m.id);

    let lcm = monkeys
        .iter()
        .map(|m| m.divisible_by)
        .reduce(|a, b| a * b)
        .unwrap();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            let items: Vec<u64> = monkeys[i].items.drain(0..).collect();

            for item in items {
                let result = (monkeys[i].operation)(item);
                let throw_to_id = (monkeys[i].test)(result);

                if monkeys[throw_to_id as usize].id != throw_to_id {
                    panic!("what?")
                }

                monkeys[throw_to_id as usize].items.push(result % lcm);
                monkeys[i].inspected += 1;
            }
        }
    }

    let mut inspected: Vec<u64> = monkeys.iter().map(|m| m.inspected).collect();
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
        let expected = 2713310158;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
