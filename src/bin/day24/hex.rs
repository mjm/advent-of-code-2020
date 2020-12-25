use std::collections::HashSet;

use nom::{Finish, IResult};
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{char, newline};
use nom::combinator::{all_consuming, value};
use nom::multi::{many1, separated_list1};
use itertools::Itertools;

pub struct Map {
    black_tiles: HashSet<(i32, i32, i32)>,
}

#[derive(Debug, Clone)]
pub enum Direction {
    East,
    Southeast,
    Southwest,
    West,
    Northwest,
    Northeast,
}

impl Map {
    pub fn new() -> Self {
        Map {
            black_tiles: HashSet::new(),
        }
    }

    pub fn flip_at(&mut self, x: i32, y: i32, z: i32) {
        let point = (x, y, z);
        if self.black_tiles.contains(&point) {
            self.black_tiles.remove(&point);
        } else {
            self.black_tiles.insert(point);
        }
    }

    pub fn flip(&mut self, moves: &[Direction]) {
        let (mut x, mut y, mut z) = (0, 0, 0);

        for dir in moves {
            dir.move_point(&mut x, &mut y, &mut z);
        }

        self.flip_at(x, y, z);
    }

    pub fn count_black_tiles(&self) -> usize {
        self.black_tiles.len()
    }

    pub fn flip_by_rules(&mut self) {
        let mut tiles_to_remove = HashSet::new();
        let mut tiles_to_add = HashSet::new();

        for point in &self.black_tiles {
            let black_neighbors = point_neighbors(point).into_iter()
                .filter(|p| self.black_tiles.contains(p))
                .count();
            if black_neighbors == 0 || black_neighbors > 2 {
                tiles_to_remove.insert(point.clone());
            }
        }

        for point in self.black_tiles.iter().flat_map(|p| point_neighbors(p)) {
            if self.black_tiles.contains(&point) {
                continue;
            }

            let black_neighbors = point_neighbors(&point).into_iter()
                .filter(|p| self.black_tiles.contains(p))
                .count();
            if black_neighbors == 2 {
                tiles_to_add.insert(point);
            }
        }

        for p in tiles_to_remove {
            self.black_tiles.remove(&p);
        }
        for p in tiles_to_add {
            self.black_tiles.insert(p);
        }
    }
}

impl Direction {
    fn all() -> [Self; 6] {
        use Direction::*;
        [East, Southeast, Southwest, West, Northwest, Northeast]
    }

    fn move_point(&self, x: &mut i32, y: &mut i32, z: &mut i32) {
        match self {
            Self::East => {
                *x += 1;
                *y -= 1;
            }
            Self::Southeast => {
                *z += 1;
                *y -= 1;
            }
            Self::Southwest => {
                *z += 1;
                *x -= 1;
            }
            Self::West => {
                *y += 1;
                *x -= 1;
            }
            Self::Northwest => {
                *y += 1;
                *z -= 1;
            }
            Self::Northeast => {
                *x += 1;
                *z -= 1;
            }
        }
    }
}

fn point_neighbors(point: &(i32, i32, i32)) -> Vec<(i32, i32, i32)> {
    Direction::all().iter().map(|dir| {
        let (mut x, mut y, mut z) = *point;
        dir.move_point(&mut x, &mut y, &mut z);
        (x, y, z)
    }).collect_vec()
}

pub fn parse_move_lists(s: &str) -> Result<Vec<Vec<Direction>>, nom::error::Error<&str>> {
    all_consuming(separated_list1(newline, parse_move_list))(s)
        .finish()
        .map(|(_, lists)| lists)
}

fn parse_move_list(s: &str) -> IResult<&str, Vec<Direction>> {
    many1(parse_direction)(s)
}

fn parse_direction(s: &str) -> IResult<&str, Direction> {
    use Direction::*;
    alt((
        value(East, char('e')),
        value(Southeast, tag("se")),
        value(Southwest, tag("sw")),
        value(West, char('w')),
        value(Northwest, tag("nw")),
        value(Northeast, tag("ne")),
    ))(s)
}