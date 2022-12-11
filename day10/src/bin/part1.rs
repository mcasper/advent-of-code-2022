use anyhow::Result;

fn main() -> Result<()> {
    let lines = lines("src/bin/input.txt".into())?;
    let result = solve(lines)?;
    println!("Day 10, part 1 result: {}", result);
    Ok(())
}

#[derive(Clone)]
enum Instruction {
    Noop,
    Addx(i64),
}

impl From<&String> for Instruction {
    fn from(s: &String) -> Instruction {
        if s == "noop" {
            return Instruction::Noop;
        }

        return Instruction::Addx(s.split(" ").last().unwrap().parse::<i64>().unwrap());
    }
}

struct Cpu {
    instructions: Vec<Instruction>,
    instruction_pointer: i64,
    current_instruction: Option<Instruction>,
    cycle: i64,
    register: i64,
    instruction_cycles: Option<u32>,
}

impl Cpu {
    fn new(instructions: Vec<Instruction>) -> Self {
        Cpu {
            instructions,
            instruction_pointer: 0,
            current_instruction: None,
            cycle: 1,
            instruction_cycles: None,
            register: 1,
        }
    }

    fn run_cycle(&mut self) {
        if self.current_instruction.is_none() {
            self.current_instruction =
                Some(self.instructions[self.instruction_pointer as usize].clone());
            self.instruction_pointer += 1;
        }

        match self.current_instruction {
            Some(Instruction::Noop) => {
                self.current_instruction = None;
            }
            Some(Instruction::Addx(v)) => {
                if self.instruction_cycles.is_none() {
                    self.instruction_cycles = Some(1);
                } else {
                    self.instruction_cycles = None;
                    self.current_instruction = None;
                    self.register += v;
                }
            }
            None => {
                unreachable!();
            }
        }

        self.cycle += 1;
    }
}

fn solve(lines: Vec<String>) -> Result<i64> {
    let instructions: Vec<Instruction> = lines.iter().map(|l| l.into()).collect();
    let mut signal_strengths: Vec<i64> = vec![];
    let mut cpu = Cpu::new(instructions);

    while cpu.cycle < 221 {
        if cpu.cycle == 20
            || cpu.cycle == 60
            || cpu.cycle == 100
            || cpu.cycle == 140
            || cpu.cycle == 180
            || cpu.cycle == 220
        {
            signal_strengths.push(cpu.register * cpu.cycle);
        }

        cpu.run_cycle();
    }

    println!("{:?}", signal_strengths);

    Ok(signal_strengths.iter().sum())
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
        let expected = 13140;
        let actual = solve(lines("src/bin/sample.txt".into()).unwrap()).unwrap();
        assert_eq!(expected, actual);
    }
}
