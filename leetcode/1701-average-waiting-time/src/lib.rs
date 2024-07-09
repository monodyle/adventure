pub fn average_waiting_time(customers: Vec<Vec<i32>>) -> f64 {
    let mut complete = -1;
    let mut waitings = vec![];
    for customer in customers {
        if complete < customer[0] {
            complete = customer[0] + customer[1]
        } else {
            complete += customer[1]
        }
        waitings.push((complete - customer[0]) as f64)
    }

    waitings.iter().sum::<f64>() / waitings.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = average_waiting_time(vec![vec![1, 2], vec![2, 5], vec![4, 3]]);
        assert_eq!(result, 5f64);
    }

    #[test]
    fn example_2() {
        let result = average_waiting_time(vec![vec![5, 2], vec![5, 4], vec![10, 3], vec![20, 1]]);
        assert_eq!(result, 3.25);
    }
}
