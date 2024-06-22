pub fn number_of_substrings(s: String) -> i32 {
    let a = b'a';
    let s = s.as_bytes();
    let mut ans = 0;
    let mut cnt = vec![0, 0, 0];

    let mut cur = 0;
    for &char in s {
        cnt[(char - a) as usize] += 1;
        while cnt.iter().all(|&c| c > 0) {
            cnt[(s[cur] - a) as usize] -= 1;
            cur += 1;
        }
        ans += cur;
    }

    ans as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = number_of_substrings("abcabc".to_string());
        assert_eq!(result, 10);
    }

    #[test]
    fn example_2() {
        let result = number_of_substrings("aaacb".to_string());
        assert_eq!(result, 3);
    }

    #[test]
    fn example_3() {
        let result = number_of_substrings("abc".to_string());
        assert_eq!(result, 1);
    }
}
