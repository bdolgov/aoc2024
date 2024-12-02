use color_eyre::eyre::Result;
use itertools::Itertools;
use std::fs;

fn is_safe<'a>(report: impl Iterator<Item = &'a i64>) -> bool {
    let distances = report
        .tuple_windows()
        .map(|(a, b)| a - b)
        .collect::<Vec<_>>();
    distances.iter().all(|x| 1i64 <= *x && *x <= 3i64)
        || distances.iter().all(|x| -3i64 <= *x && *x <= -1i64)
}

fn subsets<'a>(report: &'a Vec<i64>) -> Vec<impl Iterator<Item = &'a i64> + 'a> {
    (0..report.len())
        .map(|skipped| itertools::chain(&report[..skipped], &report[skipped + 1..]))
        .collect()
}

fn main() -> Result<()> {
    let reports = fs::read_to_string("data/day02.in")?
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .map(str::parse)
                .collect::<Result<Vec<i64>, _>>()
        })
        .collect::<Result<Vec<_>, _>>()?;

    let part_one = reports
        .iter()
        .filter(|report| is_safe(report.iter()))
        .count();
    println!("Part one: {}", part_one);

    let part_two = reports
        .iter()
        .filter(|report| subsets(*report).into_iter().any(is_safe))
        .count();
    println!("Part two: {}", part_two);

    Ok(())
}
