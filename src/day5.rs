use aoc_runner_derive::aoc;

pub struct Input {
    pages: Vec<Vec<u64>>,
    order: Vec<(u64, u64)>,
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Input {
    let (order, mut pages): (Vec<&str>, Vec<&str>) =
        input.lines().partition(|line| line.contains("|"));

    pages = pages[1..].to_vec();

    let pages_nums = pages
        .iter()
        .map(|page| {
            page.split(',')
                .map(|n| n.parse::<u64>().expect("page parsed as number"))
                .collect::<Vec<u64>>()
        })
        .collect::<Vec<Vec<u64>>>();

    let mut order_pairs = Vec::<(u64, u64)>::new();

    order.iter().for_each(|o| {
        let (u, v) = o.split_once('|').expect("order split by |");
        let prev_page: u64 = u.parse().expect("parsed prev page");
        let next_page: u64 = v.parse().expect("parsed next page");

        order_pairs.push((prev_page, next_page));
    });

    Input {
        pages: pages_nums,
        order: order_pairs,
    }
}

#[aoc(day5, part1)]
pub fn print_sum(input: &Input) -> u64 {
    let pages_nums = input.pages.clone();
    let order_pairs = input.order.clone();

    let mut sum: u64 = 0;

    for page in pages_nums {
        if is_valid_order(&order_pairs, &page) {
            // If the page numbers are in valid order, add the middle number
            sum += page[page.len() / 2];
        }
    }

    sum
}

fn is_valid_order(order: &Vec<(u64, u64)>, numbers: &[u64]) -> bool {
    order.iter().all(|(prev, next)| {
        let prev_pos = numbers.iter().position(|&x| x == *prev);
        let next_pos = numbers.iter().position(|&x| x == *next);

        if prev_pos.is_none() || next_pos.is_none() {
            return true;
        }

        prev_pos.unwrap() < next_pos.unwrap()
    })
}

#[aoc(day5, part2)]
fn correct(input: &Input) -> u64 {
    let pages_nums = input.pages.clone();
    let order_pairs = input.order.clone();

    let mut sum: u64 = 0;

    for page in pages_nums {
        if !is_valid_order(&order_pairs, &page) {
            // If the page numbers are invalid order, fix page and sum middle
            // go from right to left

            println!("incorrect page {:?}", page);

            let fixed = fix_page(page, &order_pairs);

            sum += fixed[fixed.len() / 2];
        }
    }

    sum
}

fn fix_page(page: Vec<u64>, order: &Vec<(u64, u64)>) -> Vec<u64> {
    let mut fixed = page.clone();

    for i in (0..page.len()).rev() {
        for j in (0..i).rev() {
            let prev = fixed[i];
            let next = fixed[j];

            if order.iter().any(|(p, n)| *p == prev && *n == next) {
                fixed[i] = next;
                fixed[j] = prev;
            }
        }
    }

    fixed
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    static INPUT: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn fix_page_test() {
        let input = input_generator(INPUT);
        assert_eq!(
            vec![97, 75, 47, 61, 53],
            fix_page(vec![75, 97, 47, 61, 53], &input.order)
        );

        assert_eq!(vec![61, 29, 13], fix_page(vec![61, 13, 29], &input.order));
    }

    #[test]
    fn test_part1() {
        assert_eq!(143, print_sum(&input_generator(INPUT)));
    }

    #[test]
    fn test_part2() {
        assert_eq!(123, correct(&input_generator(INPUT)));
    }
}
