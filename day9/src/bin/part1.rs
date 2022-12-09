use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 9, part 1 result: {}", result);
    Ok(())
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

struct Move {
    direction: Direction,
    quantity: i64,
}

impl From<&String> for Move {
    fn from(s: &String) -> Self {
        let parts: Vec<&str> = s.split(" ").collect();
        let quantity = parts[1].parse::<i64>().unwrap();

        match parts[0] {
            "U" => {
                return Move {
                    direction: Direction::Up,
                    quantity,
                }
            }
            "D" => {
                return Move {
                    direction: Direction::Down,
                    quantity,
                }
            }
            "L" => {
                return Move {
                    direction: Direction::Left,
                    quantity,
                }
            }
            "R" => {
                return Move {
                    direction: Direction::Right,
                    quantity,
                }
            }
            _ => {
                unreachable!()
            }
        }
    }
}

#[derive(Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

fn move_head(head_pos: &mut Point, m: &Move) {
    match m.direction {
        Direction::Up => {
            head_pos.y += m.quantity;
        }
        Direction::Down => {
            head_pos.y -= m.quantity;
        }
        Direction::Right => {
            head_pos.x += m.quantity;
        }
        Direction::Left => {
            head_pos.x -= m.quantity;
        }
    }
}

fn tail_needs_to_move(tail_pos: &Point, head_pos: &Point) -> bool {
    (head_pos.x - tail_pos.x).abs() > 1 || (head_pos.y - tail_pos.y).abs() > 1
}

fn move_tail(tail_pos: &mut Point, head_pos: &Point) {
    let dx = (head_pos.x - tail_pos.x).abs();
    let dy = (head_pos.y - tail_pos.y).abs();

    if dx == 0 && dy == 0 {
        unreachable!()
    }

    if dx >= 1 {
        if head_pos.x > tail_pos.x {
            tail_pos.x += 1;
        } else {
            tail_pos.x -= 1;
        }
    }

    if dy >= 1 {
        if head_pos.y > tail_pos.y {
            tail_pos.y += 1;
        } else {
            tail_pos.y -= 1;
        }
    }
}

fn print_board(head_pos: &Point, tail_pos: &Point) {
    for y in -10..10 {
        for x in -10..10 {
            if head_pos.x == x && head_pos.y == y {
                print!("H ")
            } else if tail_pos.x == x && tail_pos.y == y {
                print!("T ")
            } else {
                print!(". ")
            }
        }
        print!("\n")
    }
    print!("\n");
    print!("\n");
    print!("\n");
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let moves: Vec<Move> = lines.iter().map(|l| l.into()).collect();
    let mut visited: HashMap<Point, bool> = HashMap::new();

    let mut head_position = Point { x: 0, y: 0 };
    let mut tail_position = Point { x: 0, y: 0 };

    visited.insert(tail_position.clone(), true);

    // print_board(&head_position, &tail_position);

    for m in moves {
        move_head(&mut head_position, &m);
        // print_board(&head_position, &tail_position);
        while tail_needs_to_move(&tail_position, &head_position) {
            move_tail(&mut tail_position, &head_position);
            // print_board(&head_position, &tail_position);
            visited.insert(tail_position.clone(), true);
        }
    }

    Ok(visited.keys().len() as i64)
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
