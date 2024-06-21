pub fn max_satisfied(customers: Vec<i32>, grumpy: Vec<i32>, minutes: i32) -> i32 {
    let mut total = 0;
    let mut window = 0;
    let mut max = 0;

    for i in 0..customers.len() {
        if grumpy[i] == 0 {
            total += customers[i]
        } else {
            window += customers[i]
        }
        if i >= minutes as usize && grumpy[i - minutes as usize] == 1 {
            window -= customers[i - minutes as usize]
        }
        max = max.max(window)
    }

    max + total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = max_satisfied(
            vec![1, 0, 1, 2, 1, 1, 7, 5],
            vec![0, 1, 0, 1, 0, 1, 0, 1],
            3,
        );
        assert_eq!(result, 16);
    }

    #[test]
    fn example_2() {
        let result = max_satisfied(vec![1], vec![0], 1);
        assert_eq!(result, 1);
    }

    #[test]
    fn example_3() {
        let result = max_satisfied(vec![10, 1, 7], vec![0, 0, 0], 2);
        assert_eq!(result, 18);
    }
}
