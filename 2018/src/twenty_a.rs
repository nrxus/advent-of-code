use common::extensions::cart_product;

use std::{
    collections::{HashMap, HashSet, VecDeque},
    fmt,
    str::FromStr,
};

fn solve(input: &str) -> u32 {
    let house = House::from_str(input).unwrap();
    house.most_steps()
}

impl House {
    fn most_steps(&self) -> u32 {
        let mut explored = HashSet::new();
        let mut frontier = VecDeque::new();
        frontier.push_back((0, self.start));
        let mut max_distance = 0;

        while let Some((distance, position)) = frontier.pop_front() {
            max_distance = distance;
            explored.insert(position);

            let rooms: Vec<_> = self
                .adjacent_rooms(position)
                .iter()
                .filter_map(|x| *x)
                .filter(|x| !explored.contains(x))
                .filter(|x| !frontier.iter().any(|(_, f)| f == x))
                .map(|x| (distance + 1, x))
                .collect();

            frontier.extend(rooms);
        }

        max_distance
    }

    fn adjacent_rooms(&self, pos: usize) -> [Option<usize>; 4] {
        //house is padded with walls so we do not need to do edge-checking for out of bounds
        let mut rooms = [None, None, None, None];

        //up
        match self.tiles[pos - self.cols] {
            Tile::HDoor | Tile::VDoor => rooms[0] = Some(pos - 2 * self.cols),
            _ => {}
        }

        //down
        match self.tiles[pos + self.cols] {
            Tile::HDoor | Tile::VDoor => rooms[1] = Some(pos + 2 * self.cols),
            _ => {}
        }

        //left
        match self.tiles[pos - 1] {
            Tile::HDoor | Tile::VDoor => rooms[2] = Some(pos - 2),
            _ => {}
        }

        //right
        match self.tiles[pos + 1] {
            Tile::HDoor | Tile::VDoor => rooms[3] = Some(pos + 2),
            _ => {}
        }
        rooms
    }
}

#[derive(Debug)]
struct House {
    tiles: Vec<Tile>,
    cols: usize,
    start: usize,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Wall,
    Room,
    HDoor,
    VDoor,
}

impl FromStr for House {
    type Err = String;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let input = input.trim().as_bytes();
        if input[0] != b'^' {
            return Err(String::from("did not start with '^'"));
        }
        if input[input.len() - 1] != b'$' {
            return Err(String::from("did not end with '$'"));
        }

        let insert_tile = |l: (isize, isize), map: &mut HashMap<_, _>, tile: Tile| {
            if let Some(r) = map.insert(l, tile) {
                if r != tile {
                    return Err(format!(
                        "overriding room at {:?} from {:?} to {:?}",
                        l, r, tile
                    ));
                }
            }
            Ok(())
        };

        let room_adder = |mut l: (isize, isize),
                          map: &mut HashMap<_, _>,
                          door: Tile,
                          delta: (isize, isize)|
         -> Result<_, String> {
            l.0 += delta.0;
            l.1 += delta.1;
            insert_tile(l, map, door)?;
            l.0 += delta.0;
            l.1 += delta.1;
            insert_tile(l, map, Tile::Room)?;
            Ok(l)
        };

        let input = &input[1..input.len() - 1];

        let mut map = HashMap::with_capacity(input.len() * 2);
        map.insert((0, 0), Tile::Room);
        let mut room_walker = HashSet::new();
        room_walker.insert((0, 0));
        let mut room_stack: Vec<(HashSet<(isize, isize)>, HashSet<(isize, isize)>)> = vec![];

        input.iter().try_for_each(|c| -> Result<_, String> {
            match c {
                b'N' => {
                    room_walker = room_walker
                        .iter()
                        .map(|&l| room_adder(l, &mut map, Tile::VDoor, (0, -1)))
                        .collect::<Result<_, _>>()?
                }
                b'S' => {
                    room_walker = room_walker
                        .iter()
                        .map(|&l| room_adder(l, &mut map, Tile::VDoor, (0, 1)))
                        .collect::<Result<_, _>>()?
                }
                b'E' => {
                    room_walker = room_walker
                        .iter()
                        .map(|&l| room_adder(l, &mut map, Tile::HDoor, (1, 0)))
                        .collect::<Result<_, _>>()?
                }
                b'W' => {
                    room_walker = room_walker
                        .iter()
                        .map(|&l| room_adder(l, &mut map, Tile::HDoor, (-1, 0)))
                        .collect::<Result<_, _>>()?
                }
                b'(' => {
                    room_stack.push((room_walker.clone(), HashSet::new()));
                }
                b'|' => {
                    let last_level = room_stack
                        .last_mut()
                        .ok_or("found | with an empty room stack")?;
                    last_level.1 = last_level.1.union(&room_walker).cloned().collect();
                    room_walker = last_level.0.clone();
                }
                b')' => {
                    let last_level = room_stack.pop().ok_or("found ) with an empty room stack")?;
                    room_walker = last_level.1.union(&room_walker).cloned().collect();
                }
                _ => Err(format!("did not expect {}", c))?,
            }
            Ok(())
        })?;

        // pad edges with walls
        let min_x = map.keys().map(|(x, _)| *x).min().unwrap() - 1;
        let max_x = map.keys().map(|(x, _)| *x).max().unwrap() + 1;
        let min_y = map.keys().map(|(_, y)| *y).min().unwrap() - 1;
        let max_y = map.keys().map(|(_, y)| *y).max().unwrap() + 1;

        let tiles = cart_product(min_y..=max_y, min_x..=max_x)
            .map(|(y, x)| map.get(&(x, y)).cloned().unwrap_or(Tile::Wall))
            .collect();

        let cols = (max_x - min_x) as usize + 1;
        let start = (-min_x) as usize + (-min_y) as usize * cols;
        Ok(House { tiles, cols, start })
    }
}

impl fmt::Display for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tiles.iter().enumerate().try_for_each(|(i, a)| {
            let c = match a {
                Tile::HDoor => '|',
                Tile::VDoor => '-',
                Tile::Room if i == self.start => 'X',
                Tile::Room => '.',
                Tile::Wall => '#',
            };

            if i % self.cols == self.cols - 1 {
                writeln!(f, "{}", c)
            } else {
                write!(f, "{}", c)
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"^WNE$";
        let house = House::from_str(input).unwrap();
        let stringified = r"#####
#.|.#
#-###
#.|X#
#####";
        assert_eq!(house.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 3);
    }

    #[test]
    fn test_b() {
        let input = r"^ENWWW(NEEE|SSE(EE|N))$";
        let house = House::from_str(input).unwrap();
        let stringified = r"#########
#.|.|.|.#
#-#######
#.|.|.|.#
#-#####-#
#.#.#X|.#
#-#-#####
#.|.|.|.#
#########";
        assert_eq!(house.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn test_c() {
        let input = r"^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
        let house = House::from_str(input).unwrap();
        let stringified = r"###########
#.|.#.|.#.#
#-###-#-#-#
#.|.|.#.#.#
#-#####-#-#
#.#.#X|.#.#
#-#-#####-#
#.#.|.|.|.#
#-###-###-#
#.|.|.#.|.#
###########";
        assert_eq!(house.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 18);
    }

    #[test]
    fn test_d() {
        let input = r"^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
        let house = House::from_str(input).unwrap();
        let stringified = r"#############
#.|.|.|.|.|.#
#-#####-###-#
#.#.|.#.#.#.#
#-#-###-#-#-#
#.#.#.|.#.|.#
#-#-#-#####-#
#.#.#.#X|.#.#
#-#-#-###-#-#
#.|.#.|.#.#.#
###-#-###-#-#
#.|.#.|.|.#.#
#############";
        assert_eq!(house.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 23);
    }

    #[test]
    fn test_e() {
        let input = r"^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";
        let house = House::from_str(input).unwrap();
        let stringified = r"###############
#.|.|.|.#.|.|.#
#-###-###-#-#-#
#.|.#.|.|.#.#.#
#-#########-#-#
#.#.|.|.|.|.#.#
#-#-#########-#
#.#.#.|X#.|.#.#
###-#-###-#-#-#
#.|.#.#.|.#.|.#
#-###-#####-###
#.|.#.|.|.#.#.#
#-#-#####-#-#-#
#.#.|.|.|.#.|.#
###############";
        assert_eq!(house.to_string().trim(), stringified.trim());
        assert_eq!(solve(input), 31);
    }
}

common::read_main!();
