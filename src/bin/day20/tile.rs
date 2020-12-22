use enumflags2::BitFlags;
use itertools::Itertools;
use nom::{Finish, IResult};
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::combinator::{all_consuming, map, map_res};
use nom::lib::std::collections::HashMap;
use nom::multi::many1;
use nom::sequence::{pair, preceded, terminated};

#[derive(BitFlags, Copy, Clone, Debug, PartialEq)]
#[repr(u8)]
pub enum Side {
    Top = 0b0001,
    Bottom = 0b0010,
    Left = 0b0100,
    Right = 0b1000,
}

impl Side {
    fn rotated(&self, n: u32) -> Self {
        if n == 0 {
            return *self;
        }

        (match self {
            Self::Top => Self::Right,
            Self::Right => Self::Bottom,
            Self::Bottom => Self::Left,
            Self::Left => Self::Top,
        }).rotated(n - 1)
    }

    fn rotated_inv(&self, n: u32) -> Self {
        if n == 0 {
            return *self;
        }

        self.rotated(4 - n)
    }

    fn flipped(&self) -> Side {
        match self {
            Self::Top => Self::Right,
            Self::Right => Self::Top,
            Self::Bottom => Self::Left,
            Self::Left => Self::Bottom,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Edge {
    pub tile_id: u32,
    pub value: u16,
    pub side: Side,
    pub flipped: bool,
}

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u32,
    pub width: usize,
    pub height: usize,
    data: Vec<char>,
}

impl Tile {
    pub fn new(id: u32, s: &str) -> Tile {
        let width = s.lines().next().unwrap().chars().count();
        let height = s.lines().count();
        let data = s.lines().flat_map(|s| s.chars()).collect();

        Tile {
            id,
            width,
            height,
            data,
        }
    }

    pub fn from_raw_list(s: &str) -> Result<Vec<Tile>, nom::error::Error<&str>> {
        all_consuming(parse_tiles)(s).finish().map(|(_, tiles)| tiles)
    }

    pub fn all_edges(&self) -> Vec<Edge> {
        use Side::*;

        [Top, Bottom, Left, Right].iter().flat_map(|side| {
            vec![self.get_edge(*side, false), self.get_edge(*side, true)]
        }).collect()
    }

    pub fn find_corners(tiles: &[Tile]) -> Vec<&Tile> {
        let mut edges_by_value = HashMap::new();
        let mut tiles_by_id = HashMap::new();

        for tile in tiles {
            tiles_by_id.insert(tile.id, tile);
            for edge in tile.all_edges() {
                match edges_by_value.get_mut(&edge.value) {
                    None => { edges_by_value.insert(edge.value, vec![edge]); }
                    Some(edges) => { edges.push(edge); }
                }
            }
        }

        let grouped_by_tile = edges_by_value.values()
            .filter_map(|edges| if edges.len() == 1 { Some((edges[0].tile_id, &edges[0])) } else { None })
            .into_group_map();

        grouped_by_tile.iter().filter(|(_, v)| v.len() == 4)
            .map(|(id, _)| tiles_by_id.get(id).unwrap())
            .cloned()
            .collect_vec()
    }

    fn get_edge(&self, side: Side, flipped: bool) -> Edge {
        use Side::*;
        let value = match side {
            Top => Tile::get_edge_value(self.data.iter().take(self.width), flipped, self.width),
            Bottom => Tile::get_edge_value(self.data.iter().skip((self.height - 1) * self.width).rev(), flipped, self.width),
            Left => Tile::get_edge_value(self.data.iter().step_by(self.width).rev(), flipped, self.height),
            Right => Tile::get_edge_value(self.data.iter().skip(self.width - 1).step_by(self.width), flipped, self.height),
        };

        Edge {
            tile_id: self.id,
            value,
            side,
            flipped,
        }
    }

    fn get_edge_value<'a, Iter: Iterator<Item=&'a char>>(iter: Iter, flipped: bool, total: usize) -> u16 {
        let mut value = 0;

        for (i, c) in iter.enumerate() {
            if *c == '#' {
                let shift_idx = if flipped { total - 1 - i } else { i };
                value = value | (1 << shift_idx);
            }
        }

        value
    }
}

pub struct TileView {
    tile: Tile,
    flip: bool,
    rotations: u32,
}

impl TileView {
    pub fn new(tile: Tile) -> Self {
        TileView {
            tile,
            flip: false,
            rotations: 0,
        }
    }

    pub fn rotate(&mut self, n: u32) {
        self.rotations += n;
    }

    pub fn flip(&mut self) {
        self.flip = !self.flip;
    }

    pub fn get_edge(&self, side: Side) -> Edge {
        let mut real_side = side.rotated_inv(self.rotations);
        if self.flip {
            real_side = real_side.flipped();
        }
        self.tile.get_edge(real_side, self.flip)
    }
}

fn parse_tiles(s: &str) -> IResult<&str, Vec<Tile>> {
    many1(terminated(parse_tile, tag("\n\n")))(s)
}

fn parse_tile(s: &str) -> IResult<&str, Tile> {
    map(
        pair(
            preceded(tag("Tile "), terminated(parse_digit1, tag(":\n"))),
            take_until("\n\n"),
        ),
        |(id, s)| Tile::new(id, s),
    )(s)
}

fn parse_digit1(s: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse())(s)
}