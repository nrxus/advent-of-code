use std::collections::HashSet;

#[derive(Clone, Copy, Debug)]
enum Track {
    Horizontal,
    Vertical,
    LeftCurve,
    RightCurve,
    Intersection,
    Empty,
}

#[derive(Debug)]
struct Cart {
    position: Coord,
    direction: Direction,
    orientation: Orientation,
    turn: Turn,
}

impl Cart {
    fn new(position: Coord, orientation: Orientation, direction: Direction) -> Self {
        Cart {
            position,
            direction,
            orientation,
            turn: Turn::Left,
        }
    }

    fn advance(&mut self) {
        match (&self.direction, &self.orientation) {
            (Direction::Negative, Orientation::Horizontal) => self.position.x -= 1,
            (Direction::Negative, Orientation::Vertical) => self.position.y -= 1,
            (Direction::Positive, Orientation::Horizontal) => self.position.x += 1,
            (Direction::Positive, Orientation::Vertical) => self.position.y += 1,
        }
    }

    fn turn(&mut self) {
        match self.turn {
            Turn::Left => {
                if let Orientation::Horizontal = self.orientation {
                    self.direction.flip();
                }
                self.orientation.flip();
            }
            Turn::Right => {
                if let Orientation::Vertical = self.orientation {
                    self.direction.flip();
                }
                self.orientation.flip();
            }
            Turn::Straight => {}
        }
        self.turn.next();
    }
}

#[derive(Debug)]
enum Orientation {
    Horizontal,
    Vertical,
}

impl Orientation {
    fn flip(&mut self) {
        *self = match self {
            Orientation::Horizontal => Orientation::Vertical,
            Orientation::Vertical => Orientation::Horizontal,
        }
    }
}

#[derive(Debug)]
enum Direction {
    Positive,
    Negative,
}

impl Direction {
    fn flip(&mut self) {
        *self = match self {
            Direction::Positive => Direction::Negative,
            Direction::Negative => Direction::Positive,
        }
    }
}

#[derive(Debug)]
enum Turn {
    Left,
    Straight,
    Right,
}

impl Turn {
    fn next(&mut self) {
        *self = match self {
            Turn::Left => Turn::Straight,
            Turn::Straight => Turn::Right,
            Turn::Right => Turn::Left,
        }
    }
}

fn solve(input: &str) -> Coord {
    let mut lines = input.lines();
    let cols = lines.next().unwrap().len();
    let rows = lines.count() + 1;
    let mut map = Vec::with_capacity(cols * rows);
    let mut carts = vec![];

    input
        .lines()
        .enumerate()
        .flat_map(|(row, l)| {
            l.chars()
                .enumerate()
                .map(move |(col, c)| (Coord { x: col, y: row }, c))
        })
        .for_each(|(coord, t)| {
            let t = match t {
                ' ' => Track::Empty,
                '/' => Track::RightCurve,
                '\\' => Track::LeftCurve,
                '-' => Track::Horizontal,
                '|' => Track::Vertical,
                '+' => Track::Intersection,
                '^' => {
                    carts.push(Cart::new(coord, Orientation::Vertical, Direction::Negative));
                    Track::Vertical
                }
                '>' => {
                    carts.push(Cart::new(
                        coord,
                        Orientation::Horizontal,
                        Direction::Positive,
                    ));
                    Track::Horizontal
                }
                '<' => {
                    carts.push(Cart::new(
                        coord,
                        Orientation::Horizontal,
                        Direction::Negative,
                    ));
                    Track::Horizontal
                }
                'v' => {
                    carts.push(Cart::new(coord, Orientation::Vertical, Direction::Positive));
                    Track::Vertical
                }
                _ => panic!("did not expect {:?}", t),
            };
            map.push(t);
        });

    dbg!(&cols);
    let mut coords: HashSet<_> = carts.iter().map(|c| c.position).collect();
    loop {
        dbg!(&carts);
        carts.sort_by_key(|c| c.position);
        let crash = carts.iter_mut().find_map(|c| {
            coords.remove(&c.position);
            let Coord { x, y } = c.position;
            match map[y * cols + x] {
                Track::Empty => panic!("cart should not be on empty space"),
                Track::Horizontal | Track::Vertical => {}
                Track::LeftCurve => c.orientation.flip(),
                Track::RightCurve => {
                    c.orientation.flip();
                    c.direction.flip();
                }
                Track::Intersection => c.turn(),
            }
            c.advance();
            if coords.contains(&c.position) {
                Some(c.position)
            } else {
                coords.insert(c.position);
                None
            }
        });
        if let Some(p) = crash {
            break p;
        }
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy, Hash)]
struct Coord {
    y: usize,
    x: usize,
}

use std::fmt;

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let input = r"/->-\        ".to_owned() + r"
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   ";
        assert_eq!(solve(&input), Coord { x: 7, y: 3 });
    }
}

common::read_main!();
