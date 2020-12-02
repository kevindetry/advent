use std::{collections::HashSet, error::Error};

use advent::get_input;

const RESULTING_SUM: i32 = 2020;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input(file!())?
        .lines()
        .filter_map(|x| x.parse().ok())
        .collect::<HashSet<_>>();
    if let Some(result) = solve_first_problem(&input) {
        println!("{:?}", result);
    }
    if let Some(result) = solve_second_problem(&input) {
        println!("{:?}", result);
    }
    Ok(())
}

fn solve_first_problem(input: &HashSet<i32>) -> Option<i32> {
    input
        .iter()
        .find_map(|&x| input.get(&(RESULTING_SUM - x)).map(|&y| x * y))
}

fn solve_second_problem(input: &HashSet<i32>) -> Option<i32> {
    input.iter().find_map(|&x| {
        input
            .iter()
            .find_map(|&y| input.get(&(RESULTING_SUM - x - y)).map(|&z| x * y * z))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn get_input() -> HashSet<i32> {
        [1721, 979, 366, 299, 675, 1456].iter().cloned().collect()
    }

    #[test]
    fn first_problem() {
        assert_eq!(solve_first_problem(&get_input()).unwrap(), 514579)
    }

    #[test]
    fn second_problem() {
        assert_eq!(solve_second_problem(&get_input()).unwrap(), 241861950)
    }
}
