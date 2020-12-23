mod food;

use advent_of_code_2020::read_input;
use crate::food::IngredientList;
use nom::lib::std::collections::HashMap;
use std::collections::HashSet;
use itertools::Itertools;

fn main() {
    let contents = read_input();
    let lists = IngredientList::from_lines(&contents).unwrap();
    let allergens = IngredientList::identify_allergens(&lists[..]);

    part1(&lists[..], &allergens);
    part2(&allergens);
}

fn part1(lists: &[IngredientList], allergens: &HashMap<&str, &str>) {
    let ings_to_ignore: HashSet<&str> = allergens.values().cloned().collect();
    let mut total: u32 = 0;
    for list in lists {
        for ing in &list.ingredients[..] {
            if !ings_to_ignore.contains(&ing[..]) {
                total += 1;
            }
        }
    }

    println!("The total occurrences of non-allergen ingredients is {}", total);
}

fn part2(allergens: &HashMap<&str, &str>) {
    let mut entries = allergens.iter().collect_vec();
    entries.sort_by_key(|(a, _)| **a);
    let danger_list = entries.iter().map(|(_, i)| **i).join(",");
    println!("The canonical dangerous ingredient list is: {}", danger_list);
}