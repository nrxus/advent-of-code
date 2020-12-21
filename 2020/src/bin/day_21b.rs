use regex::Regex;
use std::collections::{HashMap, HashSet};

fn solve(foods: &str) -> String {
    let regex = Regex::new(r"(?P<ingredients>.*) \(contains (?P<allergens>.*)\)").unwrap();
    let mut ingredients_by_allergen = foods
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
            HashMap::new(),
            |mut ingredients_by_allergen, (ingredients, allergens)| {
                let ingredients: HashSet<_> = ingredients.collect();

                allergens.for_each(|a| {
                    ingredients_by_allergen
                        .entry(a)
                        .and_modify(|i: &mut HashSet<_>| *i = &*i & &ingredients)
                        .or_insert_with(|| ingredients.clone());
                });

                ingredients_by_allergen
            },
        );

    let mut allergic_ingredients = vec![];

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
        allergic_ingredients.push(AllergicIngredient {
            ingredient,
            allergen,
        });
    }

    debug_assert!(ingredients_by_allergen.is_empty());
    allergic_ingredients.sort();
    allergic_ingredients
        .into_iter()
        .map(|ai| ai.ingredient)
        .collect::<Vec<_>>()
        .join(",")
}

#[derive(PartialEq, Eq)]
struct AllergicIngredient<'s> {
    ingredient: &'s str,
    allergen: &'s str,
}

impl Ord for AllergicIngredient<'_> {
    fn cmp(&self, rhs: &Self) -> std::cmp::Ordering {
        self.allergen.cmp(rhs.allergen)
    }
}

impl PartialOrd for AllergicIngredient<'_> {
    fn partial_cmp(&self, rhs: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(rhs))
    }
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
        assert_eq!(solve(input), "mxmxvkd,sqjhc,fvjkl");
    }
}

common::read_main!();
