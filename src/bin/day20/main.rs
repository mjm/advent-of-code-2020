mod tile;
mod image;

use advent_of_code_2020::read_input;
use crate::tile::Tile;
use crate::image::ImageBuilder;

fn main() {
    let contents = read_input();
    let tiles = Tile::from_raw_list(&contents).unwrap();

    part1(&tiles);
    part2(&tiles);
}

fn part1(tiles: &Vec<Tile>) {
    let corner_tiles = Tile::find_corners(&tiles[..]);
    let result: u64 = corner_tiles.iter().map(|t| t.id as u64).product();
    println!("The product of the IDs of the 4 corners is {}", result);
}

fn part2(tiles: &Vec<Tile>) {
    let mut image_builder = ImageBuilder::new(&tiles[..]);
    image_builder.fill_all_slots();
    let image = image_builder.build();
    let bmp_image = image.render();
    bmp_image.save("day20.bmp").unwrap();

    let count_black = count_color(&bmp_image, bmp::consts::BLACK);
    let count_green = count_color(&bmp_image, bmp::consts::GREEN);

    println!("The number of filled in pixels that are not covered by a sea monster is {}", count_black);
    println!("The number of pixels occupied by a sea monster is {}", count_green);
}

fn count_color(img: &bmp::Image, color: bmp::Pixel) -> usize {
    let mut total = 0;

    for x in 0..img.get_width() {
        for y in 0..img.get_height() {
            if img.get_pixel(x, y) == color {
                total += 1;
            }
        }
    }

    total
}