use std::collections::HashMap;

use common::read_main;

#[cfg(test)]
const MAX_COORD: i32 = 20;

#[cfg(not(test))]
const MAX_COORD: i32 = 4_000_000;

fn solve(input: &str) -> u64 {
    let readings: HashMap<_, _> = input
        .trim()
        .lines()
        .map(|reading| {
            let reading = reading.strip_prefix("Sensor at x=").unwrap();
            let (sensor_x, reading) = reading.split_once(',').unwrap();
            let reading = reading.strip_prefix(" y=").unwrap();
            let (sensor_y, reading) = reading.split_once(':').unwrap();
            let reading = reading.strip_prefix(" closest beacon is at x=").unwrap();
            let (beacon_x, reading) = reading.split_once(',').unwrap();
            let beacon_y = reading.strip_prefix(" y=").unwrap();
            let sensor: (i32, i32) = (sensor_x.parse().unwrap(), sensor_y.parse().unwrap());
            let beacon: (i32, i32) = (beacon_x.parse().unwrap(), beacon_y.parse().unwrap());
            let distance_to_beacon = sensor.0.abs_diff(beacon.0) + sensor.1.abs_diff(beacon.1);
            (sensor, distance_to_beacon)
        })
        .collect();

    readings
        .iter()
        .find_map(|(center, distance)| {
            let edge = distance + 1;
            let ne = (0..edge).map(|i| (center.0 + (edge - i) as i32, center.1 - i as i32));
            let nw = (0..edge).map(|i| (center.0 - (i as i32), center.1 - (edge - i) as i32));
            let sw = (0..edge).map(|i| (center.0 - (edge - i) as i32, center.1 + i as i32));
            let se = (0..edge).map(|i| (center.0 + (i as i32), center.1 + (edge - i) as i32));
            let mut edge_points = ne.chain(nw).chain(sw).chain(se).filter_map(|(x, y)| {
                if (0..=MAX_COORD).contains(&x) && (0..=MAX_COORD).contains(&y) {
                    Some((x, y))
                } else {
                    None
                }
            });

            edge_points.find(|point| {
                readings.iter().filter(|(other, _)| *other != center).all(
                    |(other_center, other_distance)| {
                        let other_to_edge =
                            other_center.0.abs_diff(point.0) + other_center.1.abs_diff(point.1);
                        other_to_edge > *other_distance
                    },
                )
            })
        })
        .map(|(x, y): (i32, i32)| x as u64 * 4_000_000 + y as u64)
        .unwrap()
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
        assert_eq!(solve(input), 56000011);
    }
}

read_main!();
