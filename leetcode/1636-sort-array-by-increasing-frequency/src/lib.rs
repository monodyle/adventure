pub fn frequency_sort(mut nums: Vec<i32>) -> Vec<i32> {
    let mut feq = std::collections::HashMap::<i32, i32>::new();
    nums.iter().for_each(|&x| *(feq.entry(x).or_insert(0)) += 1);
    nums.sort_unstable_by_key(|x| (feq[x], -(*x)));
    nums
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = frequency_sort(vec![1,1,2,2,2,3]);
        assert_eq!(result, vec![3,1,1,2,2,2]);
    }

    #[test]
    fn example_2() {
        let result = frequency_sort(vec![2,3,1,3,2]);
        assert_eq!(result, vec![1,3,3,2,2]);
    }

    #[test]
    fn example_3() {
        let result = frequency_sort(vec![-1,1,-6,4,5,-6,1,4,1]);
        assert_eq!(result, vec![5,-1,4,4,-6,-6,1,1,1]);
    }
}
