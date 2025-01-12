pub fn can_be_valid(s: String, locked: String) -> bool {
    if s.len() % 2 != 0 {
        return false
    }

    let mut left = 0;
    for (c, is_locked) in s.chars().zip(locked.chars()) {
        if is_locked == '0' || c == '(' {
            left += 1;
        } else if c == ')' {
            left -= 1;
        }
        if left < 0 {
            return false;
        }
    }

    let mut right = 0;
    for (c, is_locked) in s.chars().rev().zip(locked.chars().rev()) {
        if is_locked == '0' || c == ')' {
            right += 1;
        } else if c == '(' {
            right -= 1;
        }
        if right < 0 {
            return false;
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = can_be_valid("))()))".to_owned(), "010100".to_owned());
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = can_be_valid("()()".to_owned(), "0000".to_owned());
        assert_eq!(result, true);
    }

    #[test]
    fn example_3() {
        let result = can_be_valid(")".to_owned(), "0".to_owned());
        assert_eq!(result, false);
    }
}
