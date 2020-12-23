use std::collections::HashMap;

use nom::{Finish, IResult};
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char, newline};
use nom::combinator::{all_consuming, map};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, terminated};
use nom::lib::std::collections::HashSet;
use std::thread::sleep;

pub struct IngredientList {
    pub ingredients: Vec<String>,
    allergens: Vec<String>,
}

impl IngredientList {
    pub fn from_lines(s: &str) -> Result<Vec<IngredientList>, nom::error::Error<&str>> {
        all_consuming(
            separated_list1(newline, parse_ingredient_list)
        )(s).finish().map(|(_, lists)| lists)
    }

    pub fn identify_allergens(lists: &[IngredientList]) -> HashMap<&str, &str> {
        let mut valid_ingredients = HashMap::new();

        for list in lists {
            let ingredients: HashSet<&str> = (&list.ingredients[..]).iter().map(|s| &s[..]).collect();
            for allergen in &list.allergens[..] {
                match valid_ingredients.get_mut(allergen) {
                    None => {
                        valid_ingredients.insert(allergen, ingredients.clone());
                    },
                    Some(ings) => {
                        *ings = ings.iter().filter(|i| ingredients.contains(**i)).cloned().collect();
                    }
                }
            }
        }

        let mut assignments = HashMap::new();

        while valid_ingredients.values().any(|ings| ings.len() > 0) {
            match valid_ingredients.iter().filter_map(|(allergen, ings)| {
                if ings.len() == 1 {
                    ings.iter().next().map(|ing| (*allergen, *ing))
                } else {
                    None
                }
            }).next() {
                None => { panic!("Oh no! We still have unassigned allergens but there aren't any left with exactly one ingredient option"); },
                Some((allergen, ing)) => {
                    for (_, v) in valid_ingredients.iter_mut() {
                        v.remove(ing);
                    }

                    assignments.insert(&(*allergen)[..], ing);
                }
            }

            sleep(std::time::Duration::from_millis(200));
        }

        assignments
    }
}

fn parse_ingredient_list(s: &str) -> IResult<&str, IngredientList> {
    map(
        separated_pair(
            separated_list1(char(' '), alpha1),
            tag(" (contains "),
            terminated(
                separated_list1(tag(", "), alpha1),
                char(')'),
            )),
        |(ingredients, allergens): (Vec<&str>, Vec<&str>)| {
            IngredientList {
                ingredients: ingredients.into_iter().map(String::from).collect(),
                allergens: allergens.into_iter().map(String::from).collect(),
            }
        },
    )(s)
}
