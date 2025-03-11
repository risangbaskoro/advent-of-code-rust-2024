use glam::IVec2;
use itertools::Itertools;

fn parse(input: &str) -> Vec<(IVec2, char)> {
    input
        .lines()
        .enumerate()
        .flat_map(move |(y, line)| {
            line.chars().enumerate().map(
                move |(x, char)| {
                    (IVec2::new(x as i32, y as i32), char)
                },
            )
        })
        .filter(|(_, char)| *char != '.')
        .collect()
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let antennas = parse(input);
    let outer_bound = 0..input.lines().count() as i32;

    let result = antennas
        .iter()
        .combinations(2)
        .filter_map(|pair| {
            let ((a, a_char), (b, b_char)) =
                (pair[0], pair[1]);

            if a_char != b_char {
                return None;
            }

            let mut antinodes = vec![2 * a - b, 2 * b - a];
            antinodes.retain(|antinode| {
                outer_bound.contains(&antinode.x)
                    && outer_bound.contains(&antinode.y)
            });

            Some(antinodes)
        })
        .flatten()
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
