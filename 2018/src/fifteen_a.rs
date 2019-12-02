use std::{
    collections::{HashSet, VecDeque},
    fmt,
    str::FromStr,
};

fn solve(input: &str) -> u32 {
    let mut field: BattleField = input.parse().unwrap();
    let mut rounds = 0;
    loop {
        if field.round() {
            rounds += 1;
        } else {
            break;
        }
    }
    rounds * field.health()
}

struct BattleField {
    tiles: Vec<Tile>,
    num_cols: usize,
}

impl BattleField {
    /// returns true if it was able to complete a whole round before any unit observed no enemies
    fn round(&mut self) -> bool {
        let num_tiles = self.tiles.len();
        let mut units = HashSet::new();
        let mut elf_count = self.tiles.iter().filter(|t| t.is_elf()).count();
        let mut goblin_count = self.tiles.iter().filter(|t| t.is_goblin()).count();

        if elf_count == 0 || goblin_count == 0 {
            return false;
        }

        for i in 0..num_tiles {
            match self.tiles[i] {
                Tile::Unit(u) => {
                    if elf_count == 0 || goblin_count == 0 {
                        return false;
                    }
                    if !units.insert(u.id) {
                        continue;
                    }
                    let movement = self.movement(i, |e| u.is_enemy(e));
                    let pos = if let Some(pos) = movement {
                        self.tiles.swap(i, pos);
                        pos
                    } else {
                        i
                    };
                    let attack = self.attack(pos, |e| u.is_enemy(e));
                    if let Some(pos) = attack {
                        if let Tile::Unit(ref mut unit) = &mut self.tiles[pos] {
                            match unit.health.checked_sub(3) {
                                None | Some(0) => {
                                    match unit.kind {
                                        UnitKind::Goblin => goblin_count -= 1,
                                        UnitKind::Elf => elf_count -= 1,
                                    }
                                    self.tiles[pos] = Tile::Open;
                                }
                                Some(h) => unit.health = h,
                            }
                        }
                    }
                }
                _ => continue,
            }
        }

        true
    }

    fn health(&self) -> u32 {
        self.tiles
            .iter()
            .filter_map(|t| match t {
                Tile::Unit(u) => Some(u32::from(u.health)),
                _ => None,
            })
            .sum()
    }

    fn attack(&self, i: usize, is_enemy: impl Fn(&Unit) -> bool) -> Option<usize> {
        self.neighbors(i)
            .iter()
            .filter_map(|n| *n)
            .filter(|&n| {
                if let Tile::Unit(ref u) = self.tiles[n] {
                    is_enemy(u)
                } else {
                    false
                }
            })
            .min_by(|&a, &b| {
                let (health_a, health_b) = match (&self.tiles[a], &self.tiles[b]) {
                    (Tile::Unit(unit_a), Tile::Unit(unit_b)) => (unit_a.health, unit_b.health),
                    _ => unreachable!(),
                };
                if health_a == health_b {
                    a.cmp(&b)
                } else {
                    health_a.cmp(&health_b)
                }
            })
    }

    fn movement(&self, i: usize, is_enemy: impl Fn(&Unit) -> bool) -> Option<usize> {
        let mut paths: Vec<(usize, usize)> = vec![];
        let mut min_distance = None;
        let mut explored = HashSet::new();
        explored.insert(i);
        let mut frontier: VecDeque<_> = self
            .neighbors_frontier(i, 1, None)
            .iter()
            .filter_map(|&n| n)
            .collect();

        while let Some((position, distance, first_step)) = frontier.pop_front() {
            if let Some(min_distance) = min_distance {
                if distance > min_distance {
                    break;
                }
            }
            explored.insert(position);
            match self.tiles[position] {
                Tile::Open => {
                    if paths.is_empty() {
                        let neighbors =
                            self.neighbors_frontier(position, distance + 1, Some(first_step));
                        let neighbors = neighbors
                            .iter()
                            .filter_map(|&n| n)
                            .filter(|(n, _, _)| !explored.contains(n))
                            .filter(|n| !frontier.iter().any(|f| n == f))
                            .collect::<Vec<_>>();
                        frontier.extend(neighbors);
                    }
                }
                Tile::Unit(ref u) if is_enemy(u) => {
                    if distance == 1 {
                        break;
                    }
                    min_distance = Some(distance);
                    paths.push((position, first_step));
                }
                _ => {}
            }
        }

        paths
            .into_iter()
            .min_by(|a, b| if a.0 == b.0 { a.1.cmp(&b.1) } else { a.cmp(&b) })
            .map(|(_, p)| p)
    }

    fn neighbors_frontier(
        &self,
        position: usize,
        distance: usize,
        first_step: Option<usize>,
    ) -> [Option<(usize, usize, usize)>; 4] {
        let neighbors = self.neighbors(position);
        let mut frontier = [None; 4];
        neighbors
            .iter()
            .enumerate()
            .filter_map(|(i, n)| n.map(|n| (i, n)))
            .for_each(|(i, n)| {
                frontier[i] = Some((n, distance, first_step.unwrap_or(n)));
            });
        frontier
    }

    fn neighbors(&self, position: usize) -> [Option<usize>; 4] {
        let mut neighbors = [None; 4];
        if position % self.num_cols > 0 {
            neighbors[0] = Some(position - 1);
        };
        if position % self.num_cols < self.num_cols - 1 {
            neighbors[1] = Some(position + 1);
        }
        if let Some(i) = position.checked_sub(self.num_cols) {
            neighbors[2] = Some(i);
        }
        if position + self.num_cols < self.tiles.len() {
            neighbors[3] = Some(position + self.num_cols);
        }
        neighbors
    }
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
    Unit(Unit),
}

#[derive(Debug, Clone, Copy)]
struct Unit {
    health: u8,
    id: usize,
    kind: UnitKind,
}

impl Unit {
    fn is_enemy(&self, other: &Self) -> bool {
        match (&self.kind, &other.kind) {
            (UnitKind::Goblin, UnitKind::Elf) => true,
            (UnitKind::Elf, UnitKind::Goblin) => true,
            _ => false,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum UnitKind {
    Goblin,
    Elf,
}

impl Tile {
    fn is_elf(&self) -> bool {
        match self {
            Tile::Unit(u) => u.kind == UnitKind::Elf,
            _ => false,
        }
    }

    fn is_goblin(&self) -> bool {
        match self {
            Tile::Unit(u) => u.kind == UnitKind::Goblin,
            _ => false,
        }
    }
}

impl FromStr for BattleField {
    type Err = !;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut lines = input.lines();
        let num_cols = lines.next().unwrap().len();
        let tiles = input
            .lines()
            .flat_map(str::chars)
            .enumerate()
            .map(|(i, t)| match t {
                '#' => Tile::Wall,
                '.' => Tile::Open,
                'G' => Tile::Unit(Unit {
                    id: i,
                    health: 200,
                    kind: UnitKind::Goblin,
                }),
                'E' => Tile::Unit(Unit {
                    id: i,
                    health: 200,
                    kind: UnitKind::Elf,
                }),
                _ => panic!("did not expect '{}'", t),
            })
            .collect();
        Ok(BattleField { tiles, num_cols })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";
        assert_eq!(solve(input), 27730);
    }

    #[test]
    fn test_b() {
        let input = r"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";
        assert_eq!(solve(input), 36334);
    }

    #[test]
    fn test_c() {
        let input = r"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";
        assert_eq!(solve(input), 39514);
    }

    #[test]
    fn test_d() {
        let input = r"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";
        assert_eq!(solve(input), 27755);
    }

    #[test]
    fn test_e() {
        let input = r"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";
        assert_eq!(solve(input), 28944);
    }

    #[test]
    fn test_f() {
        let input = r"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";
        assert_eq!(solve(input), 18740);
    }
}

impl fmt::Display for BattleField {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.tiles.iter().enumerate().try_for_each(|(i, t)| {
            let c = match t {
                Tile::Wall => '#',
                Tile::Open => '.',
                Tile::Unit(u) => match u.kind {
                    UnitKind::Goblin => 'G',
                    UnitKind::Elf => 'E',
                },
            };

            if i % self.num_cols == self.num_cols - 1 {
                writeln!(f, "{}", c)
            } else {
                write!(f, "{}", c)
            }
        })
    }
}

common::read_main!();
//common::bootstrap!(15);
