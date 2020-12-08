use std::collections::HashSet;

fn solve(instructions: &str) -> i32 {
    let instructions: Vec<_> = instructions
        .trim()
        .lines()
        .map(|l| {
            let mut instructions = l.split_whitespace();
            let opcode = instructions.next().unwrap();
            if opcode == "nop" {
                return Instruction::Nop;
            }
            let mut arg = instructions.next().unwrap();
            if &arg[0..1] == "+" {
                arg = &arg[1..];
            }
            match opcode {
                "acc" => Instruction::Acc(arg.parse().unwrap()),
                "jmp" => Instruction::Jmp(arg.parse().unwrap()),
                o => panic!("unexpected opcode: {:?}", o),
            }
        })
        .collect();

    let mut previous_stack_pointers = HashSet::new();
    let mut stack_pointer = 0;
    let mut accumulator = 0;

    while previous_stack_pointers.insert(stack_pointer) {
        match instructions[stack_pointer] {
            Instruction::Nop => stack_pointer += 1,
            Instruction::Acc(acc) => {
                accumulator += acc;
                stack_pointer += 1;
            }
            Instruction::Jmp(jmp) => {
                stack_pointer = (stack_pointer as isize + jmp) as usize;
            }
        }
    }

    accumulator
}

#[derive(Debug)]
enum Instruction {
    Nop,
    Acc(i32),
    Jmp(isize),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
        assert_eq!(solve(input), 5);
    }
}

common::read_main!();
