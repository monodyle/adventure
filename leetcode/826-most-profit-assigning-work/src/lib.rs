pub fn max_profit_assignment(
    difficulty: Vec<i32>,
    profit: Vec<i32>,
    mut worker: Vec<i32>,
) -> i32 {
    /* if difficulty.is_empty() || worker.is_empty() {
        return 0;
    } */

    let mut jobs = difficulty
        .into_iter()
        .zip(profit.into_iter())
        .collect::<Vec<(i32, i32)>>();
    jobs.sort_unstable_by_key(|&j| j.1);

    worker.sort_unstable_by_key(|&w| -w);

    worker.iter().fold(0, |acc, &w| {
        while jobs.last().is_some() && jobs[jobs.len() - 1].0 > w {
            jobs.pop();
        }
        if let Some(&job) = jobs.last() {
            return acc + job.1;
        }
        acc
    })
}

pub fn max_profit_assignment_2(difficulty: Vec<i32>, profit: Vec<i32>, worker: Vec<i32>) -> i32 {
    let min = *difficulty.iter().min().unwrap() as usize - 1;
    let max = *difficulty.iter().max().unwrap() as usize;
    let mut earn = vec![0; max - min + 1];
    for (&d, &p) in difficulty.iter().zip(profit.iter()) {
        let i = d as usize - min;
        earn[i] = earn[i].max(p);
    }
    for i in 2..earn.len() {
        earn[i] = earn[i].max(earn[i - 1])
    }
    worker.iter().fold(0, |acc, &w| {
        let i = (w as usize).max(min).min(max) - min;
        acc + earn[i]
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
