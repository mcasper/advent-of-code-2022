use std::{collections::HashMap, iter::Peekable, ops::Index, str::Chars};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 12, part 1 result: {}", result);
    Ok(())
}

#[derive(Clone, Debug)]
enum Value {
    Integer(i64),
    Array(Vec<Value>),
}

impl Value {
    fn value_at_index(&self, index: usize) -> Option<Value> {
        match &self {
            Value::Integer(_) => panic!("uh oh"),
            Value::Array(inner) => return inner.get(index).map(|v| v.clone()),
        }
    }
}

impl From<&String> for Value {
    fn from(s: &String) -> Self {
        parse_array(&mut s.chars().peekable())
    }
}

fn parse_array(chars: &mut Peekable<Chars>) -> Value {
    let mut arr: Vec<Value> = vec![];
    let mut parsing = true;
    let mut parsing_int = String::new();

    assert!(chars.next().unwrap() == '[');

    while parsing {
        match chars.peek().unwrap() {
            '[' => arr.push(parse_array(chars)),
            ']' => {
                if parsing_int != "" {
                    arr.push(Value::Integer(parsing_int.parse::<i64>().unwrap()));
                    parsing_int = String::new();
                }

                parsing = false;
                chars.next();
            }
            ',' => {
                if parsing_int != "" {
                    arr.push(Value::Integer(parsing_int.parse::<i64>().unwrap()));
                    parsing_int = String::new();
                }
                chars.next();
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let c = chars.next();
                parsing_int.push(c.unwrap());
            }
            _ => {
                unreachable!()
            }
        }
    }

    Value::Array(arr)
}

fn pair_in_correct_order(left: Value, right: Value) -> bool {
    compare_lists(left, right).unwrap()
}

fn compare_lists(left: Value, right: Value) -> Option<bool> {
    println!("compare lists: {:?}, {:?}", left, right);
    let mut i = 0;

    loop {
        let left_item = left.value_at_index(i);
        let right_item = right.value_at_index(i);

        i += 1;

        if left_item.is_none() && right_item.is_some() {
            return Some(true);
        }

        if right_item.is_none() && left_item.is_some() {
            return Some(false);
        }

        if left_item.is_none() && right_item.is_none() {
            return None;
        }

        match (left_item.unwrap(), right_item.unwrap()) {
            (Value::Integer(left_i), Value::Integer(right_i)) => {
                println!("Comparing two integers: {}, {}", left_i, right_i);
                if left_i < right_i {
                    println!("returning true");
                    return Some(true);
                } else if right_i < left_i {
                    println!("returning false");
                    return Some(false);
                }
            }
            (Value::Array(left_list), Value::Array(right_list)) => {
                println!("Comparing two lists: {:?}, {:?}", left_list, right_list);
                if let Some(result) =
                    compare_lists(Value::Array(left_list), Value::Array(right_list))
                {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
            (Value::Integer(left_i), Value::Array(right_list)) => {
                println!("Comparing integer and list: {:?}, {:?}", left_i, right_list);
                if let Some(result) = compare_lists(
                    Value::Array(vec![Value::Integer(left_i)]),
                    Value::Array(right_list),
                ) {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
            (Value::Array(left_list), Value::Integer(right_i)) => {
                println!("Comparing list and integer: {:?}, {:?}", left_list, right_i);
                if let Some(result) = compare_lists(
                    Value::Array(left_list),
                    Value::Array(vec![Value::Integer(right_i)]),
                ) {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
        }
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let mut result = 0;

    for (i, chunk) in lines.chunks(2).enumerate() {
        let left: Value = chunk.first().unwrap().into();
        let right: Value = chunk.last().unwrap().into();
        println!("left: {:?}, right: {:?}", left, right);

        if pair_in_correct_order(left, right) {
            result += (i + 1) as i64;
        }
    }

    Ok(result)
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
        let expected = 13;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
