use core::fmt;
use std::fmt::Write;

use common::read_main;

fn solve(input: &str) -> Crt {
    let mut mid_pos = 1;
    let mut cycle = 1;
    let mut crt = Crt {
        pixels: [false; WIDTH * HEIGHT],
    };

    let instructions = input.trim().lines().map(|instruction| {
        instruction
            .split_once(' ')
            .map(|(_, num)| num.parse::<i8>().unwrap())
    });

    for instruction in instructions {
        crt.pixels[cycle - 1] = {
            let drawn = ((cycle - 1) % 40) as i8;
            ((mid_pos - 1)..=(mid_pos + 1)).contains(&drawn)
        };

        cycle += 1;
        if cycle > 240 {
            break;
        }

        let Some(num) = instruction else {
            continue;
        };

        crt.pixels[cycle - 1] = {
            let drawn = ((cycle - 1) % 40) as i8;
            ((mid_pos - 1)..=(mid_pos + 1)).contains(&drawn)
        };

        mid_pos += num as i8;

        cycle += 1;
        if cycle > 240 {
            break;
        }
    }

    if cycle <= 240 {
        crt.pixels[cycle - 1] = {
            let drawn = ((cycle - 1) % 40) as i8;
            ((mid_pos - 1)..=(mid_pos + 1)).contains(&drawn)
        };
    }

    crt
}

const WIDTH: usize = 40;
const HEIGHT: usize = 6;

struct Crt {
    pixels: [bool; WIDTH * HEIGHT],
}

impl fmt::Display for Crt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (0..HEIGHT).try_for_each(|y| {
            (0..WIDTH).try_for_each(|x| {
                let c = if self.pixels[y * WIDTH + x] { '#' } else { '.' };
                f.write_char(c)
            })?;
            f.write_char('\n')
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
";
        assert_eq!(
            solve(input).to_string(),
            "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
"
        );
    }
}

read_main!();
