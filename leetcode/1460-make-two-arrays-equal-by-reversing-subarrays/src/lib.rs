pub fn can_be_equal(mut target: Vec<i32>, mut arr: Vec<i32>) -> bool {
    target.sort();
    arr.sort();
    target == arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = can_be_equal(vec![1,2,3,4], vec![2,4,1,3]);
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = can_be_equal(vec![7], vec![7]);
        assert_eq!(result, true);
    }

    #[test]
    fn example_3() {
        let result = can_be_equal(vec![3,7,9], vec![3,7,11]);
        assert_eq!(result, false);
    }
}
