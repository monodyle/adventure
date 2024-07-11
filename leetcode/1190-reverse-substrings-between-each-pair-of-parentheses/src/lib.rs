pub fn reverse_parentheses(s: String) -> String {
    let mut stack = Vec::new();
    for c in s.chars() {
        match c {
            ')' => {
                let mut reversed = Vec::new();
                while let Some(c) = stack.pop() {
                    match c {
                        '(' => break,
                        c => reversed.push(c),
                    }
                }
                stack.extend(reversed);
            }
            c => stack.push(c),
        }
    }

    stack.into_iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = reverse_parentheses("(abcd)".to_string());
        assert_eq!(result, "dcba".to_string());
    }

    #[test]
    fn example_2() {
        let result = reverse_parentheses("(u(love)i)".to_string());
        assert_eq!(result, "iloveu".to_string());
    }

    #[test]
    fn example_3() {
        let result = reverse_parentheses("(ed(et(oc))el)".to_string());
        assert_eq!(result, "leetcode".to_string());
    }
}
