mod food;

use advent_of_code_2020::read_input;
use crate::food::IngredientList;
use nom::lib::std::collections::HashMap;
use std::collections::HashSet;

fn main() {
    let contents = read_input();
    let lists = IngredientList::from_lines(&contents).unwrap();
    let allergens = IngredientList::identify_allergens(&lists[..]);

    part1(&lists[..], &allergens);
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