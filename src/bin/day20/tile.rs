use nom::bytes::complete::{tag, take_until};
use nom::character::complete::digit1;
use nom::combinator::{map, map_res, all_consuming};
use nom::{IResult, Finish};
use nom::sequence::{pair, preceded, terminated};
use nom::multi::many1;
use nom::lib::std::collections::HashMap;
use itertools::Itertools;

#[derive(Copy, Clone, Debug)]
pub enum Side {
    Top,
    Bottom,
    Left,
    Right,
}

#[derive(Debug)]
pub struct Edge {
    tile_id: u32,
    value: u16,
    side: Side,
    flipped: bool,
}

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
                    None => { edges_by_value.insert(edge.value, vec![edge]); },
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
            Bottom => Tile::get_edge_value(self.data.iter().skip((self.height - 1) * self.width), flipped, self.width),
            Left => Tile::get_edge_value(self.data.iter().step_by(self.width), flipped, self.height),
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