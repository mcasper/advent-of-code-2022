use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 8, part 1 result: {}", result);
    Ok(())
}

struct Trees {
    inner: Vec<Vec<usize>>,
}

impl Trees {
    fn visible_trees(&self) -> usize {
        let mut result = 0;
        for y in 0..self.inner.len() {
            let row = &self.inner[y];
            for x in 0..row.len() {
                if x == 0 || y == 0 || x == (row.len() - 1) || y == (self.inner.len() - 1) {
                    result += 1;
                    continue;
                }

                let tree = &row[x];

                // left
                let mut left_visible = true;
                for xx in 0..x {
                    let other_tree = &row[xx];
                    if other_tree >= tree {
                        left_visible = false;
                    }
                }

                // right
                let mut right_visible = true;
                for xx in (x + 1)..row.len() {
                    let other_tree = &row[xx];
                    if other_tree >= tree {
                        right_visible = false;
                    }
                }

                // up
                let mut up_visible = true;
                for yy in 0..y {
                    let other_tree = &self.inner[yy][x];
                    if other_tree >= tree {
                        up_visible = false;
                    }
                }

                // down
                let mut down_visible = true;
                for yy in (y + 1)..self.inner.len() {
                    let other_tree = &self.inner[yy][x];
                    if other_tree >= tree {
                        down_visible = false;
                    }
                }

                if left_visible || right_visible || up_visible || down_visible {
                    result += 1;
                }
            }
        }

        return result;
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
        let expected = 21;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
