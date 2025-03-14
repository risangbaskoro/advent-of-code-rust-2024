use miette::miette;
use nom::{
    character::complete::{self, space1},
    multi::separated_list1,
    IResult,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, mut numbers) = parse(input)
        .map_err(|e| miette!("Error parsing: {e}"))?;

    for _ in 0..25 {
        numbers = transform(&numbers);
    }

    Ok(numbers.len().to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    separated_list1(space1, complete::u64)(input)
}

fn transform(numbers: &[u64]) -> Vec<u64> {
    numbers
        .iter()
        .flat_map(|&n| {
            let tmp = n.to_string();
            let len = tmp.len();
            if n == 0 {
                Vec::from([1])
            } else if len % 2 == 0 {
                let mid = len / 2;
                let (l, r) = tmp.split_at(mid);
                Vec::from([
                    l.parse::<u64>().unwrap(),
                    r.parse::<u64>().unwrap(),
                ])
            } else {
                Vec::from([n * 2024])
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
