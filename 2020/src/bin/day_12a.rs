fn solve(input: &str) -> u32 {
    let ship = input.trim().lines().fold(
        Ship {
            angle: 0,
            position: (0, 0),
        },
        |mut ship, instruction| {
            match &instruction[0..1] {
                "N" => ship.north(instruction[1..].parse().unwrap()),
                "S" => ship.south(instruction[1..].parse().unwrap()),
                "E" => ship.east(instruction[1..].parse().unwrap()),
                "W" => ship.west(instruction[1..].parse().unwrap()),
                "L" => ship.turn(360 - instruction[1..].parse::<u16>().unwrap()),
                "R" => ship.turn(instruction[1..].parse().unwrap()),
                "F" => ship.forward(instruction[1..].parse().unwrap()),
                i => panic!("unexpected instruction: {:?}", i),
            }
            ship
        },
    );

    ship.distance()
}

#[derive(Debug)]
struct Ship {
    angle: u16,
    position: (i32, i32),
}

impl Ship {
    pub fn distance(&self) -> u32 {
        self.position.0.abs() as u32 + self.position.1.abs() as u32
    }

    pub fn forward(&mut self, units: u32) {
        match self.angle {
            0 => self.position.0 += units as i32,
            180 => self.position.0 -= units as i32,
            90 => self.position.1 -= units as i32,
            270 => self.position.1 += units as i32,
            angle => panic!("bug: unexpected angle: {:?}", angle),
        }
    }

    pub fn turn(&mut self, angle: u16) {
        debug_assert!(angle % 90 == 0);
        self.angle = (self.angle + angle) % 360;
    }

    pub fn east(&mut self, units: u32) {
        self.position.0 += units as i32
    }

    pub fn west(&mut self, units: u32) {
        self.position.0 -= units as i32
    }

    pub fn north(&mut self, units: u32) {
        self.position.1 += units as i32
    }

    pub fn south(&mut self, units: u32) {
        self.position.1 -= units as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_one() {
        let input = r"F10
N3
F7
R90
F11";
        assert_eq!(solve(input), 25);
    }
}

common::read_main!();
