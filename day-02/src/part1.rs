use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{self, line_ending, space1},
    multi::separated_list1,
    IResult,
};
use tracing::instrument;

type Report = Vec<i32>;

enum Direction {
    Increasing,
    Decreasing,
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, reports) = parse(input)
        .map_err(|e| miette!("parse failed: {}", e))?;

    let result = reports
        .iter()
        .filter(|report| check_safety(report).is_ok())
        .count();
    Ok(result.to_string())
}

#[instrument(ret)]
fn check_safety(report: &Report) -> Result<(), String> {
    let mut direction: Option<Direction> = None;
    for (a, b) in report.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            // Negative case
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(format!(
                        "Switched to increasing: {}, {}",
                        a, b
                    ))
                }
                _ => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!(
                            "{}, {} diff was {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        direction =
                            Some(Direction::Decreasing);
                        continue;
                    };
                }
            },
            // Positive case
            1 => match direction {
                Some(Direction::Decreasing) => {
                    return Err(format!(
                        "Switched to decreasing: {}, {}",
                        a, b
                    ))
                }
                _ => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!(
                            "{}, {} diff was {}",
                            a,
                            b,
                            diff.abs()
                        ));
                    } else {
                        direction =
                            Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            // No difference
            0 => {
                return Err(format!(
                    "{} {} diff was 0",
                    a, b
                ))
            }
            _ => panic!("Should be -1, 0, or 1."),
        }
    }
    Ok(())
}

fn parse(input: &str) -> IResult<&str, Vec<Report>> {
    separated_list1(
        line_ending,
        separated_list1(space1, complete::i32),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
