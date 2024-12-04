use color_eyre::eyre::Result;
use itertools::iproduct;
use std::fs;

fn get_matrix_element(data: &Vec<Vec<char>>, x: isize, y: isize) -> Option<char> {
    Some(
        *data
            .get(usize::try_from(x).ok()?)?
            .get(usize::try_from(y).ok()?)?,
    )
}

fn scan(data: &Vec<Vec<char>>, indexes: impl Iterator<Item = (isize, isize)>) -> Option<String> {
    indexes
        .map(|(x, y)| get_matrix_element(data, x, y))
        .collect()
}

const DIRECTIONS_1: &[(isize, isize)] = &[
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -1),
    (0, 0),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

const DIRECTIONS_2: &[[(isize, isize); 3]] = &[
    [(-1, -1), (0, 0), (1, 1)],
    [(1, 1), (0, 0), (-1, -1)],
    [(-1, 1), (0, 0), (1, -1)],
    [(1, -1), (0, 0), (-1, 1)],
];

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day04.in")?
        .split_terminator('\n')
        .map(|x| x.chars().collect())
        .collect::<Vec<Vec<char>>>();

    let n = data.len() as isize;
    let m = data[0].len() as isize;

    let part_one = iproduct!(0..n, 0..m, DIRECTIONS_1)
        .filter(|(x, y, (dx, dy))| {
            scan(&data, (0..4).map(|i| (x + i * dx, y + i * dy)))
                .as_ref()
                .map(String::as_str)
                == Some("XMAS")
        })
        .count();
    println!("Part one: {part_one}");

    let part_two = iproduct!(0..n, 0..m)
        .filter(|(x, y)| {
            DIRECTIONS_2
                .iter()
                .filter(|indexes| {
                    scan(&data, indexes.iter().map(|(dx, dy)| (x + dx, y + dy)))
                        .as_ref()
                        .map(String::as_str)
                        == Some("MAS")
                })
                .count()
                == 2
        })
        .count();
    println!("Part two: {part_two}");
    Ok(())
}
