pub fn survived_robots_healths(
    positions: Vec<i32>,
    healths: Vec<i32>,
    directions: String,
) -> Vec<i32> {
    let mut robots: Vec<usize> = (0..positions.len()).collect();
    robots.sort_unstable_by_key(|&i| positions[i]);

    let mut survive = Vec::<(usize, i32)>::new();
    let mut stack = Vec::<(usize, i32)>::new();

    for &i in robots.iter() {
        if directions.as_bytes()[i] == b'L' {
            let mut hp = healths[i];
            while hp != 0 && !stack.is_empty() {
                let last = stack.len() - 1;
                if stack[last].1 < hp {
                    stack.pop();
                    hp -= 1;
                } else if stack[last].1 > hp {
                    stack[last].1 -= 1;
                    hp = 0;
                } else {
                    stack.pop();
                    hp = 0;
                }
            }
            if hp != 0 {
                survive.push((i, hp));
            }
        } else {
            stack.push((i, healths[i]));
        }
    }

    survive.append(&mut stack);
    survive.sort_unstable();
    survive.iter().map(|&(_, hp)| hp).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = survived_robots_healths(
            vec![5, 4, 3, 2, 1],
            vec![2, 17, 9, 15, 10],
            "RRRRR".to_string(),
        );
        assert_eq!(result, vec![2, 17, 9, 15, 10]);
    }

    #[test]
    fn example_2() {
        let result =
            survived_robots_healths(vec![3, 5, 2, 6], vec![10, 10, 15, 12], "RLRL".to_string());
        assert_eq!(result, vec![14]);
    }

    #[test]
    fn example_3() {
        let result =
            survived_robots_healths(vec![1, 2, 5, 6], vec![10, 10, 11, 11], "RLRL".to_string());
        assert_eq!(result, vec![]);
    }
}
