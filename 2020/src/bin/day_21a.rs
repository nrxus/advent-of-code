use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(foods: &str) -> usize {
    let regex = Regex::new(r"(?P<ingredients>.*) \(contains (?P<allergens>.*)\)").unwrap();
    let (mut ingredients_count, mut ingredients_by_allergen) = foods
        .lines()
        .map(|meal| {
            let captures = regex.captures(meal).unwrap();
            (
                captures
                    .name("ingredients")
                    .unwrap()
                    .as_str()
                    .split_whitespace(),
                captures.name("allergens").unwrap().as_str().split(", "),
            )
        })
        .fold(
            (HashMap::new(), HashMap::new()),
            |(mut ingredients_count, mut ingredients_by_allergen), (ingredients, allergens)| {
                let ingredients: HashSet<_> = ingredients.collect();
                ingredients
                    .iter()
                    .for_each(|&i| *ingredients_count.entry(i).or_insert(0_usize) += 1);

                allergens.for_each(|a| {
                    ingredients_by_allergen
                        .entry(a)
                        .and_modify(|i: &mut HashSet<_>| *i = &*i & &ingredients)
                        .or_insert_with(|| ingredients.clone());
                });

                (ingredients_count, ingredients_by_allergen)
            },
        );

    while let Some((allergen, ingredient)) =
        ingredients_by_allergen
            .iter()
            .find_map(|(allergen, ingredients)| {
                debug_assert!(!ingredients.is_empty());
                if ingredients.len() == 1 {
                    Some((*allergen, *ingredients.iter().next().unwrap()))
                } else {
                    None
                }
            })
    {
        ingredients_by_allergen.remove(allergen);
        ingredients_by_allergen
            .values_mut()
            .for_each(|ingredients| {
                ingredients.remove(ingredient);
            });
        ingredients_count.remove(ingredient);
    }

    debug_assert!(ingredients_by_allergen.is_empty());
    ingredients_count.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        let input = r"mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
        assert_eq!(solve(input), 5);
    }
}

common::read_main!();
