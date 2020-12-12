fn solve(input: &str) -> u32 {
    let ship = input.trim().lines().fold(
        Ship {
            way_point: (10, 1),
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
    way_point: (i32, i32),
    position: (i32, i32),
}

impl Ship {
    pub fn distance(&self) -> u32 {
        self.position.0.abs() as u32 + self.position.1.abs() as u32
    }

    pub fn forward(&mut self, units: u32) {
        self.position.0 += self.way_point.0 * units as i32;
        self.position.1 += self.way_point.1 * units as i32;
    }

    pub fn turn(&mut self, angle: u16) {
        debug_assert!(angle % 90 == 0 && angle < 360);

        match angle {
            0 => {}
            90 => {
                std::mem::swap(&mut self.way_point.0, &mut self.way_point.1);
                self.way_point.1 = -self.way_point.1;
            }
            180 => {
                self.way_point.0 = -self.way_point.0;
                self.way_point.1 = -self.way_point.1;
            }
            270 => {
                std::mem::swap(&mut self.way_point.0, &mut self.way_point.1);
                self.way_point.0 = -self.way_point.0;
            }
            _ => unreachable!(),
        }
    }

    pub fn east(&mut self, units: u32) {
        self.way_point.0 += units as i32
    }

    pub fn west(&mut self, units: u32) {
        self.way_point.0 -= units as i32
    }

    pub fn north(&mut self, units: u32) {
        self.way_point.1 += units as i32
    }

    pub fn south(&mut self, units: u32) {
        self.way_point.1 -= units as i32
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
        assert_eq!(solve(input), 286);
    }
}

common::read_main!();
