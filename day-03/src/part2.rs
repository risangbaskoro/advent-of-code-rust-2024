use miette::miette;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar},
    combinator::value,
    multi::{many1, many_till},
    sequence::{delimited, separated_pair},
    IResult, Parser,
};

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (_, instructions) = parse(input)
        .map_err(|e| miette!("Error parsing: {}", e))?;
    let mut enabled = true;

    let result: u32 = instructions
        .iter()
        .map(|i| match i {
            Instruction::Mul(x, y) => match enabled {
                true => x * y,
                false => 0,
            },
            Instruction::Do => {
                enabled = true;
                0
            }
            Instruction::Dont => {
                enabled = false;
                0
            }
        })
        .sum();

    Ok(result.to_string())
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Instruction {
    Mul(u32, u32),
    Do,
    Dont,
}

fn parse_mul(input: &str) -> IResult<&str, Instruction> {
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

fn parse_instruction(
    input: &str,
) -> IResult<&str, Instruction> {
    alt((
        value(Instruction::Dont, tag("don't()")),
        value(Instruction::Do, tag("do()")),
        parse_mul,
    ))(input)
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
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!("48", process(input)?);
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
