pub fn range_sum(nums: Vec<i32>, n: i32, left: i32, right: i32) -> i32 {
    let (n, left, right) = (n as usize, left as usize, right as usize);
    let mut sums = Vec::with_capacity(n * (n + 1) / 2);
    for i in 0..n {
        let mut sum = 0;
        for num in nums.iter().take(n).skip(i) {
            sum += num;
            sums.push(sum);
        }
    }

    sums.sort();

    let mut res = 0;
    for &sum in sums.iter().take(right).skip(left - 1) {
        res += sum;
        res %= 1_000_000_007;
    }
    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = range_sum(vec![1, 2, 3, 4], 4, 1, 5);
        assert_eq!(result, 13);
    }

    #[test]
    fn example_2() {
        let result = range_sum(vec![1, 2, 3, 4], 4, 3, 4);
        assert_eq!(result, 6);
    }

    #[test]
    fn example_3() {
        let result = range_sum(vec![1, 2, 3, 4], 4, 1, 10);
        assert_eq!(result, 50);
    }
}
