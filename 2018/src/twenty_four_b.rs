use lazy_static::lazy_static;
use regex::Regex;

use std::{
    cmp::{Ordering, Reverse},
    collections::HashSet,
    num::ParseIntError,
};

fn solve(input: &str) -> usize {
    let world = World::from_str(input).unwrap();
    (0..)
        .find_map(|boost| world.clone().simulate(boost))
        .unwrap()
}

#[derive(Debug, Clone)]
struct World<'s> {
    groups: Vec<Group<'s>>,
}

impl World<'_> {
    fn simulate(mut self, boost: u32) -> Option<usize> {
        self.groups
            .iter_mut()
            .filter(|g| g.faction == Faction::Immune)
            .for_each(|g| g.dmg += boost);

        let mut previous_count: usize = self.groups.iter().map(|g| g.count).sum();

        loop {
            if self.faction_won(Faction::Infection) {
                break None;
            } else if self.faction_won(Faction::Immune) {
                break Some(self.immunition_count());
            }
            self.fight();

            // handle stalemate --- marking it as losing but that is most likely a bug
            let next_count = self.groups.iter().map(|g| g.count).sum();
            if previous_count == next_count {
                return None;
            } else {
                previous_count = next_count;
            }
        }
    }

    fn faction_won(&self, faction: Faction) -> bool {
        self.groups.iter().all(|g| g.faction == faction)
    }

    fn immunition_count(&self) -> usize {
        self.groups
            .iter()
            .filter(|g| g.faction == Faction::Immune)
            .map(|g| g.count)
            .sum()
    }

    fn fight(&mut self) {
        self.groups.sort_by(Group::targetting_order);

        self.battles()
            .into_iter()
            .for_each(|(defender, attacker)| self.battle(defender, attacker));

        self.groups.retain(|g| g.count > 0);
    }

    /// Sorted list of upcoming battles in (defender_index, attacker_index) format
    fn battles(&self) -> Vec<(usize, usize)> {
        // {defender -> attacker}
        let mut battles: Vec<_> = self
            .groups
            .iter()
            .scan(HashSet::new(), |attacked, attacker| {
                let target = attacker.choose_target(&self.groups, &attacked);
                if let Some(target) = target {
                    attacked.insert(target);
                };
                Some(target)
            })
            .enumerate()
            .filter_map(|(attacker, target)| target.map(|target| (target, attacker)))
            .collect();

        battles.sort_by_key(|(_, atk_i)| Reverse(self.groups[*atk_i].initiative));
        battles
    }

    fn battle(&mut self, defender: usize, attacker: usize) {
        let dmg = self.groups[defender].potential_damage(&self.groups[attacker]);
        self.groups[defender].damage(dmg);
    }
}

#[derive(Debug, Clone)]
struct Group<'s> {
    count: usize,
    hp: u32,
    dmg: u32,
    initiative: u32,
    faction: Faction,
    dmg_type: &'s str,
    immunities: Vec<&'s str>,
    weaknesses: Vec<&'s str>,
}

impl Group<'_> {
    fn power(&self) -> u32 {
        self.count as u32 * self.dmg
    }

    fn potential_damage(&self, attacker: &Group<'_>) -> u32 {
        if self.immunities.contains(&attacker.dmg_type) {
            0
        } else if self.weaknesses.contains(&attacker.dmg_type) {
            attacker.power() * 2
        } else {
            attacker.power()
        }
    }

    fn damage(&mut self, dmg: u32) {
        let dead_units = (dmg / self.hp) as usize;
        self.count = self.count.checked_sub(dead_units).unwrap_or(0);
    }

    fn choose_target(&self, groups: &[Group], attacked: &HashSet<usize>) -> Option<usize> {
        let defending_order = |a: &Group, b: &Group| {
            a.potential_damage(self)
                .cmp(&b.potential_damage(self))
                .then_with(|| a.power().cmp(&b.power()))
                .then_with(|| a.initiative.cmp(&b.initiative))
        };
        groups
            .iter()
            .enumerate()
            .filter(|(def_i, defender)| {
                defender.faction != self.faction && !attacked.contains(def_i)
            })
            .max_by(|(_, a), (_, b)| defending_order(a, b))
            .filter(|(_, defender)| defender.potential_damage(self) > 0)
            .map(|(i, _)| i)
    }

    fn targetting_order(&self, other: &Self) -> Ordering {
        other
            .power()
            .cmp(&self.power())
            .then_with(|| other.initiative.cmp(&self.initiative))
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Faction {
    Infection,
    Immune,
}

// vvv Parsing and other load-bearing code vvvv

impl<'s> World<'s> {
    fn from_str(input: &'s str) -> Result<Self, ParsingError> {
        lazy_static! {
            static ref WORLD_RE: Regex = Regex::new(
                r"(?s)Immune System:
(?P<immune>.*)

Infection:
(?P<infection>.*)",
            )
            .unwrap();
        }

        let captures = WORLD_RE
            .captures(input)
            .ok_or_else(|| String::from("regex failed to capture world"))?;
        let immune = captures
            .name("immune")
            .ok_or_else(|| String::from("no immune in captured world"))?
            .as_str();
        let infection = captures
            .name("infection")
            .ok_or_else(|| String::from("no infection in captured world"))?
            .as_str();

        let immune = immune.lines().map(|l| Group::from_str(l, Faction::Immune));

        let infection = infection
            .lines()
            .map(|l| Group::from_str(l, Faction::Infection));

        let groups = immune.chain(infection).collect::<Result<_, _>>()?;

        Ok(World { groups })
    }
}

impl<'s> Group<'s> {
    fn from_str(input: &'s str, faction: Faction) -> Result<Self, ParsingError> {
        lazy_static! {
            static ref GROUP_RE: Regex = Regex::new(r"(?P<count>\d+) units each with (?P<hp>\d+) hit points(?: \((?P<mods>.*)\))? with an attack that does (?P<dmg>\d+) (?P<type>.*) damage at initiative (?P<initiative>\d+)").unwrap();
        }

        let captures = GROUP_RE
            .captures(input)
            .ok_or_else(|| String::from("regex failed to capture group"))?;
        let count = captures
            .name("count")
            .ok_or_else(|| String::from("no count in captured group line"))?
            .as_str()
            .parse::<usize>()?;
        let hp = captures
            .name("hp")
            .ok_or_else(|| String::from("no hp in captured group line"))?
            .as_str()
            .parse::<u32>()?;
        let dmg = captures
            .name("dmg")
            .ok_or_else(|| String::from("no dmg in captured group line"))?
            .as_str()
            .parse::<u32>()?;
        let initiative = captures
            .name("initiative")
            .ok_or_else(|| String::from("no initiative in captured group line"))?
            .as_str()
            .parse::<u32>()?;
        let dmg_type = captures
            .name("type")
            .ok_or_else(|| String::from("no dmg_type in captured group line"))?
            .as_str();
        let mods = captures.name("mods").map(|m| m.as_str());

        lazy_static! {
            static ref IMMU_RE: Regex = Regex::new(r"immune to ([^;]+)").unwrap();
        }

        lazy_static! {
            static ref WEAK_RE: Regex = Regex::new(r"weak to ([^;]+)").unwrap();
        }

        let immunities = mods
            .and_then(|m| IMMU_RE.captures(m))
            .map(|c| {
                c.get(1)
                    .ok_or_else(|| String::from("immunity capture with no immunities"))
            })
            .transpose()?
            .map(|l| l.as_str().split(", ").collect::<Vec<_>>())
            .unwrap_or_else(|| vec![]);
        let weaknesses = mods
            .and_then(|m| WEAK_RE.captures(m))
            .map(|c| {
                c.get(1)
                    .ok_or_else(|| String::from("weakness capture with no weaknesses"))
            })
            .transpose()?
            .map(|l| l.as_str().split(", ").collect::<Vec<_>>())
            .unwrap_or_else(|| vec![]);

        Ok(Group {
            faction,
            count,
            hp,
            dmg,
            initiative,
            dmg_type,
            immunities,
            weaknesses,
        })
    }
}

#[derive(Debug)]
struct ParsingError(String);

impl From<regex::Error> for ParsingError {
    fn from(e: regex::Error) -> Self {
        ParsingError(e.to_string())
    }
}

impl From<String> for ParsingError {
    fn from(s: String) -> Self {
        ParsingError(s)
    }
}

impl From<ParseIntError> for ParsingError {
    fn from(e: ParseIntError) -> Self {
        ParsingError(e.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = r"Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4";
        assert_eq!(solve(input), 51);
    }
}

common::read_main!();
//common::bootstrap!(24);
