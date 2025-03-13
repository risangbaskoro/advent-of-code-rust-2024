use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::take_till,
    character::complete::satisfy, multi::many1,
    sequence::preceded, IResult,
};
use nom_locate::{position, LocatedSpan};

type Span<'a> = LocatedSpan<&'a str>;

fn alphanum_pos(
    input: Span,
) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) =
        satisfy(|c| c.is_alphanumeric())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(
        take_till(|c: char| c.is_alphanumeric()),
        alphanum_pos,
    ))(input)
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let height = input.lines().count();
    let width =
        input.lines().next().unwrap().chars().count();
    let y_bound = 0..height as i32;
    let x_bound = 0..width as i32;

    let (_input, mut antennas) = parse(Span::new(input))
        .map_err(|e| miette!("Error parsing: {e}"))?;

    antennas.sort_by(|a, b| a.1.cmp(&b.1));

    let result = antennas
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk.iter().combinations(2).flat_map(|ants| {
                let diff = ants[0].0 - ants[1].0;
                [ants[0].0 + diff, ants[1].0 - diff]
            })
        })
        .filter(|pos| {
            x_bound.contains(&pos.x)
                && y_bound.contains(&pos.y)
        })
        .unique()
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
