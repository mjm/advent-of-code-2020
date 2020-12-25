mod hex;

use advent_of_code_2020::read_input;
use crate::hex::{parse_move_lists, Direction, Map};

fn main() {
    let contents = read_input();
    let move_lists = parse_move_lists(&contents).unwrap();

    part1(&move_lists);
    part2(&move_lists);
}

fn part1(move_lists: &Vec<Vec<Direction>>) {
    let mut map = Map::new();
    for moves in move_lists {
        map.flip(&moves[..]);
    }

    println!("The number of black tiles after flipping tiles is {}", map.count_black_tiles());
}

fn part2(move_lists: &Vec<Vec<Direction>>) {
    let mut map = Map::new();
    for moves in move_lists {
        map.flip(&moves[..]);
    }

    for _ in 0..100 {
        map.flip_by_rules();
    }

    println!("The number of black tiles after 100 days is {}", map.count_black_tiles());
}