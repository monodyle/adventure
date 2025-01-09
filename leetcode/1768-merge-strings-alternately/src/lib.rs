pub fn merge_alternately(word1: String, word2: String) -> String {
    let mut chars1 = word1.chars();
    let mut chars2 = word2.chars();

    let mut ans = String::with_capacity(word1.len() + word2.len());

    for _ in 0..word1.len().min(word2.len()) {
        if let Some(c) = chars1.next() {
            ans.push(c);
        }
        if let Some(c) = chars2.next() {
            ans.push(c);
        }
    }

    ans.extend(chars1);
    ans.extend(chars2);

    ans
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = merge_alternately("abc".to_owned(), "pqr".to_owned());
        assert_eq!(result, "apbqcr");
    }

    #[test]
    fn example_2() {
        let result = merge_alternately("ab".to_owned(), "pqrs".to_owned());
        assert_eq!(result, "apbqrs");
    }

    #[test]
    fn example_3() {
        let result = merge_alternately("abcd".to_owned(), "pq".to_owned());
        assert_eq!(result, "apbqcd");
    }
}
