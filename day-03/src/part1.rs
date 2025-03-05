use nom::{
    bytes::complete::tag,
    character::complete::{self, anychar},
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input).unwrap();

    let result: u32 = instructions
        .iter()
        .map(|i| match i {
            Instruction::Mul(x, y) => x * y,
        })
        .sum();

    Ok(result.to_string())
}

#[derive(Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
}

fn parse_instruction(
    input: &str,
) -> IResult<&str, Instruction> {
    let (input, _) = tag("mul")(input)?;

    let (input, pair) = delimited(
        tag("("),
        separated_pair(
            complete::u32,
            tag(","),
            complete::u32,
        ),
        tag(")"),
    )(input)?;

    Ok((input, Instruction::Mul(pair.0, pair.1)))
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    let (input, result) = many1(
        many_till(anychar, parse_instruction)
            .map(|(_, ins)| ins),
    )(input)?;

    Ok((input, result))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!("161", process(input)?);
        Ok(())
    }

    #[test]
    fn test_instruction() -> miette::Result<()> {
        let input = "mul(3,2)";
        let (_, instruction) =
            match parse_instruction(input) {
                Ok(it) => it,
                _ => ("abc", Instruction::Mul(0, 0)),
            };
        assert_eq!(Instruction::Mul(3, 2), instruction);
        Ok(())
    }
}
