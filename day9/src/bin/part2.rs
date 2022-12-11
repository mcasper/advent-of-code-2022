use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 9, part 2 result: {}", result);
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

#[derive(Debug, Clone, Eq, Hash, PartialEq)]
struct Point {
    x: i64,
    y: i64,
}

struct Rope {
    head: Point,
    middle: Vec<Point>,
    tail: Point,
}

fn knot_needs_to_move(follower_pos: &Point, leader_pos: &Point) -> bool {
    (leader_pos.x - follower_pos.x).abs() > 1 || (leader_pos.y - follower_pos.y).abs() > 1
}

fn move_knot(follower_pos: &mut Point, leader_pos: &Point) {
    let dx = (leader_pos.x - follower_pos.x).abs();
    let dy = (leader_pos.y - follower_pos.y).abs();

    if dx == 0 && dy == 0 {
        unreachable!()
    }

    if dx >= 1 {
        if leader_pos.x > follower_pos.x {
            follower_pos.x += 1;
        } else {
            follower_pos.x -= 1;
        }
    }

    if dy >= 1 {
        if leader_pos.y > follower_pos.y {
            follower_pos.y += 1;
        } else {
            follower_pos.y -= 1;
        }
    }
}

fn print_board(rope: &Rope) {
    for y in (-30..30).rev() {
        'x_loop: for x in -30..30 {
            if rope.head.x == x && rope.head.y == y {
                print!("H ");
                continue 'x_loop;
            }

            for (i, middling) in rope.middle.iter().enumerate() {
                if middling.x == x && middling.y == y {
                    print!("{} ", i + 1);
                    continue 'x_loop;
                }
            }

            if rope.tail.x == x && rope.tail.y == y {
                print!("T ");
                continue 'x_loop;
            }

            print!(". ")
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

    let mut rope = Rope {
        head: Point { x: 0, y: 0 },
        middle: vec![
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
            Point { x: 0, y: 0 },
        ],
        tail: Point { x: 0, y: 0 },
    };

    visited.insert(rope.tail.clone(), true);

    for m in moves {
        for _ in 0..m.quantity {
            match m.direction {
                Direction::Up => rope.head.y += 1,
                Direction::Down => rope.head.y -= 1,
                Direction::Right => rope.head.x += 1,
                Direction::Left => rope.head.x -= 1,
            }

            for i in 0..rope.middle.len() {
                let prev_knot: Point;
                if i == 0 {
                    prev_knot = rope.head.clone();
                } else {
                    prev_knot = rope.middle[i - 1].clone();
                }

                while knot_needs_to_move(&rope.middle[i], &prev_knot) {
                    move_knot(&mut rope.middle[i], &prev_knot);
                }
            }

            while knot_needs_to_move(&rope.tail, rope.middle.last().unwrap()) {
                move_knot(&mut rope.tail, rope.middle.last().unwrap());
                visited.insert(rope.tail.clone(), true);
            }
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
        let expected = 36;
        let actual = solve(lines("src/bin/larger-sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
