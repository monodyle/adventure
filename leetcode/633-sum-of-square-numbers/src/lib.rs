pub fn judge_square_sum(mut c: i32) -> bool {
    if c <= 1 {
        return true;
    }

    let mut i = 2;
    while i * i <= c {
        let mut count = 0;
        if c % i == 0 {
            while c % i == 0 {
                count += 1;
                c /= i;
            }
            if i % 4 == 3 && count % 2 == 1 {
                return false;
            }
        }
        i += 1
    }

    // https://en.wikipedia.org/wiki/Fermat%27s_theorem_on_sums_of_two_squares
    c % 4 != 3
}

pub fn judge_square_sum_2(c: i32) -> bool {
    if c <= 1 {
        return true;
    }

    let mut a: i64 = 0;
    let mut b = (c as f32).sqrt().round() as i64;

    while a <= b {
        if a * a + b * b == c as i64 {
            return true;
        } else if a * a + b * b < c as i64 {
            a += 1;
        } else {
            b -= 1;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = judge_square_sum(5);
        assert_eq!(result, true);
    }

    #[test]
    fn example_2() {
        let result = judge_square_sum(3);
        assert_eq!(result, false);
    }

    #[test]
    fn example_3() {
        let result = judge_square_sum(4);
        assert_eq!(result, true);
    }

    #[test]
    fn example_4() {
        let result = judge_square_sum(1);
        assert_eq!(result, true);
    }

    #[test]
    fn example_5() {
        let result = judge_square_sum(0);
        assert_eq!(result, true);
    }

    #[test]
    fn example_6() {
        let result = judge_square_sum(100);
        assert_eq!(result, true);
    }

    #[test]
    fn example_7() {
        let result = judge_square_sum(1000);
        assert_eq!(result, true);
    }

    #[test]
    fn example_8() {
        let result = judge_square_sum(2147483600);
        assert_eq!(result, true);
    }
}
