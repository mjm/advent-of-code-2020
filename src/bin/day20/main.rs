mod tile;
mod image;

use advent_of_code_2020::read_input;
use crate::tile::Tile;
use crate::image::ImageBuilder;
use itertools::Itertools;

fn main() {
    let contents = read_input();
    let tiles = Tile::from_raw_list(&contents).unwrap();

    // part1(&tiles);
    part2(&tiles);
}

fn part1(tiles: &Vec<Tile>) {
    let corner_tiles = Tile::find_corners(&tiles[..]);
    let result: u64 = corner_tiles.iter().map(|t| t.id as u64).product();
    println!("The product of the IDs of the 4 corners is {}", result);
}

fn part2(tiles: &Vec<Tile>) {
    let mut image_builder = ImageBuilder::new(&tiles[..]);
    image_builder.fill_next_slot();
    image_builder.fill_next_slot();
}