pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut s = std::collections::HashMap::new();
    for (i, &v) in nums.iter().enumerate() {
        let d = target - v;
        if let Some(&j) = s.get(&d) {
            return vec![j as i32, i as i32];
        }
        s.insert(v, i);
    }
    vec![0,0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_1() {
        let result = two_sum(vec![2,7,11,15], 9);
        assert_eq!(result, vec![0,1]);
    }

    #[test]
    fn case_2() {
        let result = two_sum(vec![3,2,4], 6);
        assert_eq!(result, vec![1,2]);
    }

    #[test]
    fn case_3() {
        let result = two_sum(vec![3,3], 6);
        assert_eq!(result, vec![0,1]);
    }
}
