use std::collections::HashSet;

use common::read_main;

#[cfg(test)]
const DESIRED_Y: i32 = 10;

#[cfg(not(test))]
const DESIRED_Y: i32 = 2000000;

fn solve(input: &str) -> usize {
    let mut beacons_at_y = HashSet::new();
    let mut scanned_x_at_y = HashSet::new();

    input.trim().lines().for_each(|reading| {
        let reading = reading.strip_prefix("Sensor at x=").unwrap();
        let (sensor_x, reading) = reading.split_once(',').unwrap();
        let reading = reading.strip_prefix(" y=").unwrap();
        let (sensor_y, reading) = reading.split_once(':').unwrap();
        let reading = reading.strip_prefix(" closest beacon is at x=").unwrap();
        let (beacon_x, reading) = reading.split_once(',').unwrap();
        let beacon_y = reading.strip_prefix(" y=").unwrap();
        let sensor: (i32, i32) = (sensor_x.parse().unwrap(), sensor_y.parse().unwrap());
        let beacon: (i32, i32) = (beacon_x.parse().unwrap(), beacon_y.parse().unwrap());
        if beacon.1 == DESIRED_Y {
            beacons_at_y.insert(beacon.0);
        }
        let distance_to_beacon = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
        let distance_to_y = sensor.1.abs_diff(DESIRED_Y);
        let Some(x_range) = distance_to_beacon.checked_sub(distance_to_y) else {
            return;
        };

        let min_x = sensor.0 - (x_range as i32);
        let max_x = sensor.0 + (x_range as i32);
        scanned_x_at_y.extend(min_x..=max_x);
    });

    scanned_x_at_y.len() - beacons_at_y.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
";
        assert_eq!(solve(input), 26);
    }
}

read_main!();
