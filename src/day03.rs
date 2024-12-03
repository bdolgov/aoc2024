use color_eyre::eyre::{bail, Result};
use itertools::Itertools;
use regex::{Captures, Regex};
use std::fs;

fn multiply(captures: Captures) -> Result<i64> {
    let (_, [a, b]) = captures.extract();
    Ok(a.parse::<i64>()? * b.parse::<i64>()?)
}

fn multiply_with_enabled<'a>(captures: Captures, enabled: &mut bool) -> Result<Option<i64>> {
    Ok(
        match (
            &captures.name("mul_a"),
            &captures.name("mul_b"),
            &captures.name("do"),
            &captures.name("dont"),
        ) {
            (Some(a), Some(b), None, None) => {
                if *enabled {
                    Some(a.as_str().parse::<i64>()? * b.as_str().parse::<i64>()?)
                } else {
                    None
                }
            }
            (None, None, Some(_), None) => {
                *enabled = true;
                None
            }
            (None, None, None, Some(_)) => {
                *enabled = false;
                None
            }
            _ => {
                bail!("unexpected set of matching groups");
            }
        },
    )
}

fn main() -> Result<()> {
    let data = fs::read_to_string("data/day03.in")?;
    let part_one = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)")?
        .captures_iter(&data)
        .map(multiply)
        .process_results(|iter| iter.sum::<i64>())?;
    println!("Part one: {part_one}");

    let mut enabled = true;
    let part_two =
        Regex::new(r"mul\((?<mul_a>\d{1,3}),(?<mul_b>\d{1,3})\)|(?<do>do)\(\)|(?<dont>don't)\(\)")?
            .captures_iter(&data)
            .map(|x| multiply_with_enabled(x, &mut enabled))
            .process_results(|iter| iter.filter_map(|x| x).sum::<i64>())?;
    println!("Part two: {part_two}");
    Ok(())
}
