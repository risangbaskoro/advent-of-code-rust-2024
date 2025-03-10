use itertools::Itertools;

use miette::miette;

use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

use rayon::prelude::*;

const OPERATORS: [char; 3] = ['*', '+', '|'];

fn parse(
    input: &str,
) -> IResult<&str, Vec<(u64, Vec<u64>)>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::u64,
            tag(": "),
            separated_list1(space1, complete::u64),
        ),
    )(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, parsed) = parse(input)
        .map_err(|e| miette!("Error parsing: {e}"))?;

    let result: u64 = parsed
        .par_iter()
        .filter_map(|(test, numbers)| {
            let num_operators = numbers.len() - 1;

            (0..num_operators)
                .map(|_| OPERATORS)
                .multi_cartesian_product()
                .any(|seq| {
                    let mut s = seq.iter();

                    *test
                        == numbers
                            .iter()
                            .copied()
                            .reduce(|a, b| {
                                match s.next().unwrap() {
                                    '*' => a * b,
                                    '+' => a + b,
                                    '|' => format!(
                                        "{}{}",
                                        a, b
                                    )
                                    .parse::<u64>()
                                    .unwrap(),
                                    _ => {
                                        panic!(
                                        "Invalid operator!"
                                    )
                                    }
                                }
                            })
                            .unwrap()
                })
                .then_some(test)
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        assert_eq!("11387", process(input)?);
        Ok(())
    }
}
