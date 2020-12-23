use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};

use enumflags2::BitFlags;
use itertools::Itertools;

use crate::tile::{Edge, Side, Tile, TileView};

pub struct Image {
    tiles: HashMap<(usize, usize), TileView>,
    size: usize,
}

impl Image {
    pub fn new() -> Self {
        Image {
            tiles: HashMap::new(),
            size: 0,
        }
    }

    pub fn insert(&mut self, x: usize, y: usize, tile: TileView) {
        if x >= self.size {
            self.size = x + 1;
        }
        if y >= self.size {
            self.size = y + 1;
        }
        self.tiles.insert((x, y), tile);
    }

    pub fn get_tile(&self, x: usize, y: usize) -> Option<&TileView> {
        self.tiles.get(&(x, y))
    }

    pub fn get_neighbors(&self, x: usize, y: usize) -> Vec<(Side, &TileView)> {
        let all_sides: BitFlags<Side> = BitFlags::all();
        all_sides.iter().filter_map(|side| {
            match side {
                Side::Top => (y.checked_sub(1)).and_then(|y| self.get_tile(x, y)),
                Side::Left => (x.checked_sub(1)).and_then(|x| self.get_tile(x, y)),
                Side::Right => self.get_tile(x + 1, y),
                Side::Bottom => self.get_tile(x, y + 1),
            }.map(|tile| (side, tile))
        }).collect_vec()
    }

    pub fn get_edges(&self, x: usize, y: usize) -> Vec<Edge> {
        self.get_neighbors(x, y).into_iter().map(|(side, tile)| {
            match side {
                Side::Top => tile.get_edge(Side::Bottom),
                Side::Bottom => tile.get_edge(Side::Top),
                Side::Left => tile.get_edge(Side::Right),
                Side::Right => tile.get_edge(Side::Left),
            }
        }).collect_vec()
    }

    pub fn render(&self) -> bmp::Image {
        let tile_size = self.tiles.values().next().unwrap().size() - 2;
        let img_size = (tile_size * self.size) as u32;
        let mut img = bmp::Image::new(img_size, img_size);

        for ((x, y), tile) in &self.tiles {
            let x0 = x * tile_size;
            let y0 = y * tile_size;

            println!("Tile {} starting at ({}, {})", tile.id(), x0, y0);

            for dx in 1..=(tile_size) {
                for dy in 1..=(tile_size) {
                    let color = if tile.is_filled_at(dx, dy) { bmp::consts::BLACK } else { bmp::consts::WHITE };
                    img.set_pixel((x0 + dx - 1) as u32, (y0 + dy - 1) as u32, color);
                }
            }
        }

        img
    }
}

pub struct ImageBuilder {
    image: Image,
    slots: VecDeque<(usize, usize)>,
    tiles: HashMap<u32, Box<Tile>>,
    edges: HashMap<u16, Vec<Edge>>,
}

impl ImageBuilder {
    pub fn new(tiles: &[Tile]) -> Self {
        let mut edges_by_value = HashMap::new();
        let mut tiles_by_id = HashMap::new();

        for tile in tiles {
            tiles_by_id.insert(tile.id, Box::new((*tile).clone()));
            for edge in tile.all_edges() {
                match edges_by_value.get_mut(&edge.value) {
                    None => { edges_by_value.insert(edge.value, vec![edge]); }
                    Some(edges) => { edges.push(edge); }
                }
            }
        }

        ImageBuilder {
            image: Image::new(),
            slots: vec![(0, 0)].into(),
            tiles: tiles_by_id,
            edges: edges_by_value,
        }
    }

    pub fn fill_all_slots(&mut self) {
        while !self.slots.is_empty() {
            self.fill_next_slot();
        }
    }

    pub fn build(self) -> Image {
        self.image
    }

    fn fill_next_slot(&mut self) {
        let (x, y) = self.slots.pop_front().unwrap();

        if (0, 0) == (x, y) {
            self.fill_initial_slot();
        } else {
            self.fill_slot(x, y);
        }
    }

    fn fill_initial_slot(&mut self) {
        self.slots.push_back((0, 1));
        self.slots.push_back((1, 0));

        let (corner_tile, edges) = self.pop_corner_tile();

        println!("Found corner tile: {:?}", corner_tile);
        println!("Corner tile has edges: {:?}", edges);

        let sides = edges.iter()
            .map(|e| e.side)
            .fold(BitFlags::empty(), |sides, s| sides | s);

        // figure out how many rotations are needed to get to Left | Top
        let rotations: u32 = if sides == Side::Top | Side::Left {
            0
        } else if sides == Side::Bottom | Side::Left {
            1
        } else if sides == Side::Bottom | Side::Right {
            2
        } else if sides == Side::Top | Side::Right {
            3
        } else {
            panic!("Unexpected sides for border edges of corner tile: {:?}", sides)
        };
        println!("Need {} rotations", rotations);

        let tile_id = corner_tile.id;
        let mut tile = TileView::new(corner_tile);
        tile.rotate(rotations);

        println!("Placing tile {} at (0, 0)\n  Edges: {:?}", tile_id, tile.get_all_edges());
        self.image.insert(0, 0, tile);
    }

    fn fill_slot(&mut self, x: usize, y: usize) {
        if self.image.get_tile(x, y).is_some() {
            return;
        }

        println!("Filling slot ({}, {})", x, y);
        let mut edges_to_match = self.image.get_edges(x, y);
        assert!(edges_to_match.len() >= 1);
        println!("Finding tile to match edges: {:?}", edges_to_match);

        let first_edge = edges_to_match.pop().unwrap();
        let matching_edges = self.edges.get(&first_edge.value).unwrap().iter()
            .filter(|edge| edge.tile_id != first_edge.tile_id)
            .collect_vec();
        println!("All matching edges: {:?}", matching_edges);

        let matching_edge = self.edges.get(&first_edge.value).unwrap().iter()
            .filter(|edge| edge.tile_id != first_edge.tile_id)
            .next()
            .cloned();

        match matching_edge {
            Some(edge) => {
                println!("Found matching edge: {:?}", edge);
                let mut tile = TileView::new(self.pop_tile(&edge.tile_id));

                let mut cur_side = edge.side;
                if !edge.flipped {
                    cur_side = cur_side.flipped();
                    tile.flip();
                    println!("Flipping the tile");
                }

                while cur_side != first_edge.side.opposite() {
                    cur_side = cur_side.rotated(1);
                    tile.rotate(1);
                    println!("Rotating the tile");
                }

                println!("Placing tile {} at ({}, {})\n  Edges: {:?}", &edge.tile_id, x, y, tile.get_all_edges());
                self.image.insert(x, y, tile);
                self.slots.push_back((x + 1, y));
                self.slots.push_back((x, y + 1));
            }
            None => { return; }
        }
    }

    fn pop_corner_tile(&mut self) -> (Tile, Vec<Edge>) {
        let (tile_id, edges) = self.edges_by_tile_id().into_iter()
            .filter(|(_, v)| v.len() == 4)
            .next()
            .unwrap();

        let tile = self.pop_tile(&tile_id);
        (tile, edges)
    }

    fn pop_tile(&mut self, tile_id: &u32) -> Tile {
        *self.tiles.remove(&tile_id).unwrap()
    }

    fn edges_by_tile_id(&self) -> HashMap<u32, Vec<Edge>> {
        self.edges.values()
            .filter_map(|edges| if edges.len() == 1 { Some((edges[0].tile_id, edges[0].clone())) } else { None })
            .into_group_map()
    }
}