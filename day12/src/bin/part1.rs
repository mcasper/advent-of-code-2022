use std::collections::HashMap;

use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 12, part 1 result: {}", result);
    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq, Hash, Eq)]
struct Point {
    x: i8,
    y: i8,
}

#[derive(Clone, Debug)]
struct Path {
    moves: usize,
    position: Point,
    history: HashMap<Point, bool>,
}

#[derive(Debug)]
struct State {
    map: HashMap<i8, HashMap<i8, i8>>,
    paths: Vec<Path>,
}

impl State {
    fn valid_move(&self, old_path: &Path, new_point: &Point) -> bool {
        self.map.contains_key(&new_point.y)
            && self.map[&new_point.y].contains_key(&new_point.x)
            && self.map[&new_point.y][&new_point.x]
                - self.map[&old_path.position.y][&old_path.position.x]
                <= 1
            && !old_path.history.contains_key(new_point)
    }

    fn possible_moves(&self, old_path: &Path, dest: &Point) -> (Vec<Path>, Option<i64>) {
        let mut result: Vec<Path> = Vec::with_capacity(4);
        let mut solution: Option<i64> = None;

        // left
        let left_move = Point {
            y: old_path.position.y,
            x: old_path.position.x - 1,
        };
        if self.valid_move(&old_path, &left_move) {
            let mut history = old_path.history.clone();
            history.insert(left_move.clone(), true);
            result.push(Path {
                moves: old_path.moves + 1,
                position: left_move.clone(),
                history,
            });
            if left_move == *dest {
                println!("Solved: {}", old_path.moves + 1);
                solution = Some((old_path.moves + 1) as i64);
            }
        }

        // right
        let right_move = Point {
            y: old_path.position.y,
            x: old_path.position.x + 1,
        };
        if self.valid_move(&old_path, &right_move) {
            let mut history = old_path.history.clone();
            history.insert(right_move.clone(), true);
            result.push(Path {
                moves: old_path.moves + 1,
                position: right_move.clone(),
                history,
            });
            if right_move == *dest {
                println!("Solved: {}", old_path.moves + 1);
                solution = Some((old_path.moves + 1) as i64);
            }
        }

        // up
        let up_move = Point {
            y: old_path.position.y - 1,
            x: old_path.position.x,
        };
        if self.valid_move(&old_path, &up_move) {
            let mut history = old_path.history.clone();
            history.insert(up_move.clone(), true);
            result.push(Path {
                moves: old_path.moves + 1,
                position: up_move.clone(),
                history,
            });
            if up_move == *dest {
                println!("Solved: {}", old_path.moves + 1);
                solution = Some((old_path.moves + 1) as i64);
            }
        }

        // down
        let down_move = Point {
            y: old_path.position.y + 1,
            x: old_path.position.x,
        };
        if self.valid_move(&old_path, &down_move) {
            let mut history = old_path.history.clone();
            history.insert(down_move.clone(), true);
            result.push(Path {
                moves: old_path.moves + 1,
                position: down_move.clone(),
                history,
            });
            if down_move == *dest {
                println!("Solved: {}", old_path.moves + 1);
                solution = Some((old_path.moves + 1) as i64);
            }
        }

        (result, solution)
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let mut map: HashMap<i8, HashMap<i8, i8>> = HashMap::new();
    let mut position = Point { x: 0, y: 0 };
    let mut dest = Point { x: 0, y: 0 };

    for (y, line) in lines.iter().enumerate() {
        let mut row: HashMap<i8, i8> = HashMap::new();
        for (x, c) in line.chars().enumerate() {
            // a..z is 97-122
            match c {
                'S' => {
                    position.x = x as i8;
                    position.y = y as i8;
                    row.insert(x as i8, 0);
                }
                'E' => {
                    dest.x = x as i8;
                    dest.y = y as i8;
                    row.insert(x as i8, 25);
                }
                _ => {
                    row.insert(x as i8, ((c as u8) - 97) as i8);
                }
            }
        }
        map.insert(y as i8, row);
    }

    let mut history = HashMap::new();
    history.insert(position.clone(), true);
    let mut state = State {
        map,
        paths: vec![Path {
            moves: 0,
            position: position.clone(),
            history,
        }],
    };

    let mut rounds = 0;
    let mut solution: Option<i64> = None;
    while solution.is_none() {
        let mut new_paths: HashMap<Point, Path> = HashMap::new();

        for old_path in &state.paths {
            let (moves, s) = state.possible_moves(&old_path, &dest);

            for m in moves {
                if new_paths.contains_key(&m.position) {
                    if new_paths[&m.position].moves > m.moves {
                        new_paths.insert(m.position, m);
                    }
                } else {
                    new_paths.insert(m.position, m);
                }
            }

            if s.is_some() {
                solution = s;
            }
        }

        if new_paths.len() == 0 {
            panic!("no new paths to check");
        }

        state.paths = new_paths.into_values().collect();

        rounds += 1;
        // if rounds % 10 == 0 {
        println!("On round {}, {} possible paths", rounds, state.paths.len());
        // }
    }

    // let solution = state.solved.unwrap();
    // print_solution(&solution);
    Ok(solution.unwrap() as i64)
}

// fn print_solution(path: &Path) {
//     for p in &path.history {
//         print!("{},{} -> ", p.x, p.y);
//     }
//     println!("");
// }

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
        let expected = 31;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
