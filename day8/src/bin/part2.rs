use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 8, part 2 result: {}", result);
    Ok(())
}

struct Trees {
    inner: Vec<Vec<usize>>,
}

impl Trees {
    fn visible_trees(&self) -> usize {
        let mut max_scenic_score = 0;
        for y in 0..self.inner.len() {
            let row = &self.inner[y];
            for x in 0..row.len() {
                let tree = &row[x];

                // left
                let mut left_score = 0;
                for xx in (0..x).rev() {
                    left_score += 1;
                    let other_tree = &row[xx];
                    if other_tree >= tree {
                        break;
                    }
                }

                // right
                let mut right_score = 0;
                for xx in (x + 1)..row.len() {
                    right_score += 1;
                    let other_tree = &row[xx];
                    if other_tree >= tree {
                        break;
                    }
                }

                // up
                let mut up_score = 0;
                for yy in (0..y).rev() {
                    up_score += 1;
                    let other_tree = &self.inner[yy][x];
                    if other_tree >= tree {
                        break;
                    }
                }

                // down
                let mut down_score = 0;
                for yy in (y + 1)..self.inner.len() {
                    down_score += 1;
                    let other_tree = &self.inner[yy][x];
                    if other_tree >= tree {
                        break;
                    }
                }

                let scenic_score = left_score * right_score * up_score * down_score;
                if scenic_score > max_scenic_score {
                    max_scenic_score = scenic_score;
                }
            }
        }

        return max_scenic_score;
    }
}

fn solve(lines: Vec<String>) -> Result<usize> {
    let mut trees = Trees { inner: vec![] };
    for line in lines {
        let mut row = vec![];
        for c in line.chars() {
            let i = c.to_string().parse::<usize>()?;
            row.push(i);
        }
        trees.inner.push(row);
    }

    Ok(trees.visible_trees())
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
        let expected = 8;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
