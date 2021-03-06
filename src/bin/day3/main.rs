use std::env;
use std::fs;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();

    let input_path = &args[1];
    println!("Reading input from {}", input_path);

    let contents = fs::read_to_string(input_path)
        .expect("Something went wrong reading the file");
    let map = contents.parse::<Map>()
        .expect("The map could not be parsed");

    part1(&map);
    part2(&map);
}

fn part1(map: &Map) {
    let num_trees = map.trees_along_slope(1, 3);
    println!("Encountered {} trees before reaching the bottom.", num_trees);
}

fn part2(map: &Map) {
    let result = map.trees_along_slope(1, 1)
        * map.trees_along_slope(1, 3)
        * map.trees_along_slope(1, 5)
        * map.trees_along_slope(1, 7)
        * map.trees_along_slope(2, 1);
    println!("Product of all the numbers of trees: {}", result);
}

enum MapItem {
    Empty,
    Tree,
}

#[derive(Debug, Clone)]
struct ParseError;

impl FromStr for MapItem {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(Self::Empty),
            "#" => Ok(Self::Tree),
            _ => Err(ParseError),
        }
    }
}

struct Map {
    width: usize,
    height: usize,
    contents: Vec<MapItem>,
}

impl FromStr for Map {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().nth(0).ok_or(ParseError)?.chars().count();
        let height = s.lines().count();

        let contents = s.chars().flat_map(|c| {
            match c {
                '\n' => None,
                c => Some(c.to_string().parse::<MapItem>()),
            }
        }).collect::<Result<Vec<MapItem>, ParseError>>()?;

        Ok(Map { width, height, contents })
    }
}

impl Map {
    fn get(&self, row: usize, col: usize) -> Option<&MapItem> {
        if row >= self.height {
            return None;
        }

        let real_col = col % self.width;
        let idx = (row * self.width) + real_col;
        Some(&self.contents[idx])
    }

    fn trees_along_slope(&self, down: usize, right: usize) -> usize {
        let mut num_trees = 0;

        let mut row: usize = down;
        let mut col: usize = right;

        loop {
            match self.get(row, col) {
                Some(MapItem::Tree) => {
                    num_trees += 1;
                },
                Some(MapItem::Empty) => {},
                None => {
                    break;
                }
            };

            row += down;
            col += right;
        }

        num_trees
    }
}