pub fn is_happy_set(mut n: i32) -> bool {
    let mut seen = std::collections::HashSet::new();

    fn process(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n < 10 {
            n.pow(2)
        } else {
            (n % 10).pow(2) + process(n / 10)
        }
    }

    loop {
        if seen.contains(&n) {
            return false;
        }
        seen.insert(n);
        n = process(n);
        if n == 1 || n == 10 || n == 100 {
            return true;
        }
    }
}

pub fn is_happy_sf(n: i32) -> bool {
    fn process(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n < 10 {
            n.pow(2)
        } else {
            (n % 10).pow(2) + process(n / 10)
        }
    }

    let (mut slow, mut fast) = (n, n);
    loop {
        slow = process(slow);
        fast = process(process(fast));
        if slow == 1 || fast == 1 {
            return true;
        }
        if slow == fast {
            break;
        }
    }

    false
}

pub fn is_happy(mut n: i32) -> bool {
    fn process(n: i32) -> i32 {
        if n == 0 {
            0
        } else if n < 10 {
            n.pow(2)
        } else {
            (n % 10).pow(2) + process(n / 10)
        }
    }
    while n > 9 {
        n = process(n);
    }
    if n == 1 || n == 7 {
        true
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = is_happy(19);
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = is_happy(2);
        assert_eq!(result, false);
    }
}
