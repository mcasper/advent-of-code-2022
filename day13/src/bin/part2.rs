use std::{cmp::Ordering, fmt::Debug, iter::Peekable, str::Chars};

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 13, part 2 result: {}", result);
    Ok(())
}

#[derive(Clone, Eq)]
struct Packet {
    values: Vec<Value>,
}

impl Debug for Packet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[").unwrap();
        for value in &self.values {
            write!(f, "{:?},", value).unwrap();
        }
        write!(f, "]")
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match compare_lists(&self.values, &other.values) {
            Some(true) => {
                return Ordering::Less;
            }
            Some(false) => {
                return Ordering::Greater;
            }
            None => {
                return Ordering::Equal;
            }
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        return self.values == other.values;
    }
}

impl From<String> for Packet {
    fn from(s: String) -> Self {
        Packet {
            values: parse_array(&mut s.chars().peekable()),
        }
    }
}

#[derive(Clone, Eq)]
enum Value {
    Integer(i64),
    Array(Vec<Value>),
}

impl Debug for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self {
            Value::Integer(i) => return write!(f, "{}", i),
            Value::Array(list) => {
                write!(f, "[").unwrap();
                for v in list {
                    write!(f, "{:?},", v).unwrap();
                }
                return write!(f, "]");
            }
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Integer(left_i), Self::Integer(right_i)) => {
                return left_i == right_i;
            }
            (Self::Array(left_list), Self::Array(right_list)) => {
                return compare_lists(left_list, right_list).is_none()
            }
            (_, _) => return false,
        }
    }
}

fn parse_array(chars: &mut Peekable<Chars>) -> Vec<Value> {
    let mut arr: Vec<Value> = vec![];
    let mut parsing = true;
    let mut parsing_int = String::new();

    assert!(chars.next().unwrap() == '[');

    while parsing {
        match chars.peek().unwrap() {
            '[' => arr.push(Value::Array(parse_array(chars))),
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

    arr
}

fn compare_lists(left: &Vec<Value>, right: &Vec<Value>) -> Option<bool> {
    println!("compare lists: {:?}, {:?}", left, right);
    let mut i = 0;

    loop {
        let left_item = left.get(i);
        let right_item = right.get(i);

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
                if let Some(result) = compare_lists(left_list, right_list) {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
            (Value::Integer(left_i), Value::Array(right_list)) => {
                println!("Comparing integer and list: {:?}, {:?}", left_i, right_list);
                if let Some(result) = compare_lists(&vec![Value::Integer(*left_i)], right_list) {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
            (Value::Array(left_list), Value::Integer(right_i)) => {
                println!("Comparing list and integer: {:?}, {:?}", left_list, right_i);
                if let Some(result) = compare_lists(left_list, &vec![Value::Integer(*right_i)]) {
                    println!("returning {}", result);
                    return Some(result);
                }
            }
        }
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let divider_packet_1 = Packet {
        values: vec![Value::Array(vec![Value::Integer(2)])],
    };
    let divider_packet_2 = Packet {
        values: vec![Value::Array(vec![Value::Integer(6)])],
    };

    let mut packets: Vec<Packet> = vec![divider_packet_1.clone(), divider_packet_2.clone()];

    for line in lines {
        packets.push(line.into());
    }

    packets.sort();
    println!("sorted!");
    for packet in packets.clone() {
        println!("{:?}", packet);
    }

    let mut index_1 = 0;
    let mut index_2 = 0;
    for (i, packet) in packets.iter().enumerate() {
        if packet == &divider_packet_1 {
            index_1 = i + 1;
        }

        if packet == &divider_packet_2 {
            index_2 = i + 1;
        }
    }

    println!("indexes: {}, {}", index_1, index_2);
    Ok((index_1 * index_2) as i64)
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
        let expected = 140;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
