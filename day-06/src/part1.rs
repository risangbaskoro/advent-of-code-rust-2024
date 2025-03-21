use glam::IVec2;

const DIRECTIONS_CHAR: [char; 4] = ['^', '>', '<', 'v'];

#[derive(Debug, PartialEq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn from_char(c: char) -> Self {
        match c {
            '^' => Self::Up,
            '>' => Self::Right,
            '<' => Self::Left,
            'v' => Self::Down,
            _ => panic!("Invalid Direction: {}", c),
        }
    }

    fn to_ivec(&self) -> IVec2 {
        match self {
            Direction::Up => IVec2::NEG_Y,
            Direction::Right => IVec2::X,
            Direction::Down => IVec2::Y,
            Direction::Left => IVec2::NEG_X,
        }
    }
}

fn determine_guard_pos_dir(
    input: &str,
) -> (IVec2, Direction) {
    let mut pos = (0, 0);
    let mut dir: Direction = Direction::Up;

    for (y, row) in input.lines().enumerate() {
        if row.chars().any(|c| DIRECTIONS_CHAR.contains(&c))
        {
            let x = row
                .chars()
                .position(|c| DIRECTIONS_CHAR.contains(&c))
                .unwrap_or_default();

            let char =
                row.chars().collect::<Vec<char>>()[x];
            dir = Direction::from_char(char);
            pos = (x, y);
            break;
        };
    }

    (
        IVec2::new(pos.0 as i32, pos.1 as i32),
        dir,
    )
}

#[tracing::instrument]
pub fn process(input: &str) -> miette::Result<String> {
    let (mut pos, mut direction) =
        determine_guard_pos_dir(input);

    let map: Vec<Vec<char>> = input
        .split_terminator("\n")
        .map(|line| line.chars().collect())
        .collect();

    let (map_width, map_height) = (
        map.len() as i32,
        map.first().unwrap().len() as i32,
    );

    let mut next_pos = pos + direction.to_ivec();
    let mut visited = vec![pos, next_pos];

    while next_pos.x >= 0
        && next_pos.y >= 0
        && next_pos.y < map_height
        && next_pos.x < map_width
    {
        if map[next_pos.y as usize][next_pos.x as usize]
            == '#'
        {
            direction = match direction {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };
        } else {
            if !visited.contains(&next_pos) {
                visited.push(next_pos);
            }
            pos = next_pos
        }
        next_pos = pos + direction.to_ivec();
    }

    let result = visited.len();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use glam::IVec2;

    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }

    #[test]
    fn test_arrow_position() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!(
            (IVec2::new(4, 6), Direction::Up),
            determine_guard_pos_dir(input)
        );

        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#...>....
........#.
#.........
......#...";
        assert_eq!(
            (IVec2::new(5, 6), Direction::Right),
            determine_guard_pos_dir(input)
        );

        Ok(())
    }
}
