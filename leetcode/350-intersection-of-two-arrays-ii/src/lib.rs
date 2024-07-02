pub fn intersect(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut seen = [0; 1001];
    for &n in nums1.iter() {
        seen[n as usize] += 1;
    }

    let mut ans = vec![];
    for &n in nums2.iter() {
        if seen[n as usize] != 0 {
            ans.push(n);
            seen[n as usize] -= 1;
        }
    }

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = intersect(vec![1, 2, 2, 1], vec![2, 2]);
        assert_eq!(result, vec![2, 2]);
    }

    #[test]
    fn example_2() {
        let result = intersect(vec![4, 9, 5], vec![9, 4, 9, 8, 4]);
        assert_eq!(result, vec![9, 4]);
    }
}
