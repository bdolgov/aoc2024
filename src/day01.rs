use color_eyre::eyre::Result;
use itertools::Itertools;
use std::{collections::HashMap, fs};

fn main() -> Result<()> {
    let input = fs::read_to_string("data/day01.in")?
        .split_whitespace()
        .map(|x| x.parse::<u64>())
        .collect::<Result<Vec<_>, _>>()?;
    let (mut lhs, mut rhs): (Vec<u64>, Vec<u64>) = input.iter().tuples().unzip();
    lhs.sort();
    rhs.sort();

    let part_one = Iterator::zip(lhs.iter(), rhs.iter())
        .map(|(lhs, rhs)| lhs.abs_diff(*rhs))
        .sum::<u64>();
    println!("Part one: {}", part_one);

    let rhs = rhs
        .into_iter()
        .dedup_with_count()
        .map(|(a, b)| (b, a as u64))
        .collect::<HashMap<_, _>>();
    let part_two = lhs
        .iter()
        .map(|x| x * rhs.get(x).unwrap_or(&0u64))
        .sum::<u64>();
    println!("Part two: {}", part_two);

    Ok(())
}
