use std::{cmp::Ordering, collections::HashMap};

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};

type Rules = HashMap<u32, Vec<u32>>;
type Updates = Vec<Vec<u32>>;

fn parse_rules(input: &str) -> IResult<&str, Rules> {
    fold_many1(
        terminated(
            separated_pair(
                complete::u32,
                tag("|"),
                complete::u32,
            ),
            line_ending,
        ),
        Rules::default,
        |mut acc: Rules, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn parse_updates(input: &str) -> IResult<&str, Updates> {
    separated_list1(
        line_ending,
        separated_list1(tag(","), complete::u32),
    )(input)
}

fn parse(input: &str) -> IResult<&str, (Rules, Updates)> {
    let (input, rules) =
        terminated(parse_rules, line_ending)(input)?;
    let (input, updates) = parse_updates(input)?;

    Ok((input, (rules, updates)))
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let Ok((_, (rules, update))) = parse(input) else {
        panic!("Error parsing input!")
    };

    let indexes: Vec<usize> = update
        .iter()
        .enumerate()
        .filter_map(|(index, original_update)| {
            let mut current = original_update[0];
            let mut update = &original_update[1..];
            let mut before_pages = &original_update[0..0];

            while before_pages.len()
                != original_update.len()
            {
                if let Some(next_pages_rules) =
                    rules.get(&current)
                {
                    if !next_pages_rules.iter().all(
                        |page| !before_pages.contains(page),
                    ) {
                        return Some(index);
                    };
                }

                before_pages = &original_update
                    [0..(before_pages.len() + 1)];

                if let Some(page) = update.first() {
                    update = &update[1..];
                    current = *page;
                }
            }
            None
        })
        .collect();

    let corrected_updates: Vec<_> = indexes
        .iter()
        .map(|index| {
            let mut update = update[*index].clone();
            update.sort_by(|a, b| {
                if rules
                    .get(a)
                    .is_some_and(|pages| pages.contains(b))
                {
                    Ordering::Less
                } else {
                    Ordering::Greater
                }
            });
            update
        })
        .collect();

    let result: &u32 = &corrected_updates
        .iter()
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("123", process(input)?);
        Ok(())
    }
}
