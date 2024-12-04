use aoc_runner_derive::aoc;

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| {
                    n.parse::<i32>()
                        .unwrap_or_else(|x| panic!("error parsing {} as number", x))
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    reports.iter().filter(|report| valid_report(report)).count() as u32
}

fn valid_report(report: &[i32]) -> bool {
    let ascending = report.iter().is_sorted();
    let descending = report.iter().is_sorted_by(|a, b| a >= b);

    let mut shifted_report = report.to_vec();

    shifted_report.rotate_left(1);
    // println!("report {:?}\nshifted {:?}", report, shifted_report);
    let differ = report
        .iter()
        .zip(shifted_report.iter())
        .take(report.len() - 1)
        .all(|(a, b)| {
            let diff = a.abs_diff(*b);

            (1..=3).contains(&diff)
        });

    // println!("asc {}, desc {}, diff {}", ascending, descending, differ);

    (ascending || descending) && differ
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u32 {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|n| {
                    n.parse::<i32>()
                        .unwrap_or_else(|x| panic!("error parsing {} as number", x))
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    reports
        .iter()
        .filter(|report| valid_report(report) || damped_valid(report))
        .count() as u32
}

fn damped_valid(report: &[i32]) -> bool {
    (0..report.len()).any(|i| {
        let damped_report = report
            .iter()
            .enumerate()
            .filter_map(|(j, &num)| if j != i { Some(num) } else { None })
            .collect::<Vec<i32>>();
        valid_report(&damped_report)
    })
}

#[cfg(test)]
mod tests {
    use crate::day2::valid_report;

    // 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
    // 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
    // 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
    // 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
    // 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
    // 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
    #[test]
    fn test_report() {
        assert!(valid_report(&[7, 6, 4, 2, 1]));
        assert!(!valid_report(&[1, 2, 7, 8, 9]));
    }
}
