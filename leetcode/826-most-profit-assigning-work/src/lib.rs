pub fn max_profit_assignment(difficulty: Vec<i32>, profit: Vec<i32>, mut worker: Vec<i32>) -> i32 {
    let mut jobs = difficulty
        .into_iter()
        .zip(profit.into_iter())
        .collect::<Vec<(i32, i32)>>();
    jobs.sort_unstable_by_key(|&j| j.1);

    worker.sort_unstable_by_key(|&w| -w);

    worker.iter().fold(0, |mut acc, &w| {
        while jobs.last().is_some() && jobs[jobs.len() - 1].0 > w {
            jobs.pop();
        }
        if let Some(&job) = jobs.last() {
            acc += job.1;
        }
        acc
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = max_profit_assignment(
            vec![2, 4, 6, 8, 10],
            vec![10, 20, 30, 40, 50],
            vec![4, 5, 6, 7],
        );
        assert_eq!(result, 100);
    }

    #[test]
    fn example_2() {
        let result = max_profit_assignment(vec![85, 47, 57], vec![24, 66, 99], vec![40, 25, 25]);
        assert_eq!(result, 0);
    }
}
