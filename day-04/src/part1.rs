use std::collections::HashMap;

use glam::IVec2;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],
    [
        IVec2::new(1, -1),
        IVec2::new(2, -2),
        IVec2::new(3, -3),
    ],
    [
        IVec2::new(0, -1),
        IVec2::new(0, -2),
        IVec2::new(0, -3),
    ],
    [
        IVec2::new(-1, 1),
        IVec2::new(-2, 2),
        IVec2::new(-3, 3),
    ],
    [
        IVec2::new(-1, 0),
        IVec2::new(-2, 0),
        IVec2::new(-3, 0),
    ],
    [
        IVec2::new(-1, -1),
        IVec2::new(-2, -2),
        IVec2::new(-3, -3),
    ],
];

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let positions = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| {
            line.chars().enumerate().map(
                move |(x, value)| {
                    (IVec2::new(x as i32, y as i32), value)
                },
            )
        })
        .collect::<HashMap<IVec2, char>>();

    let mas = ['M', 'A', 'S'];
    let result: usize = positions
        .iter()
        .filter(|(_pos, value)| **value == 'X')
        .map(|(position, _value)| {
            let count = DIRECTIONS
                .iter()
                .map(|mas_pos| {
                    mas_pos
                        .iter()
                        .map(|offset| {
                            positions
                                .get(&(position + offset))
                        })
                        .enumerate()
                        .all(|(index, value)| {
                            mas.get(index) == value
                        })
                })
                .filter(|b| *b)
                .count();
            count
        })
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
