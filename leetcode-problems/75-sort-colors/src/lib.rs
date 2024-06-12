pub fn sort_colors(nums: &mut Vec<i32>) {
    let (mut r, mut w) = (0, 0);
    for n in nums.into_iter() {
        match n {
            0 => r += 1,
            1 => w += 1,
            _ => continue,
        }
    }

    for i in 0..nums.len() {
        if i < r {
            nums[i] = 0
        } else if i < r + w {
            nums[i] = 1
        } else {
            nums[i] = 2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut binding = vec![2,0,2,1,1,0];
        sort_colors(&mut binding);
        assert_eq!(binding, vec![0,0,1,1,2,2]);
    }

    #[test]
    fn example_2() {
        let mut binding = vec![2,0,1];
        sort_colors(&mut binding);
        assert_eq!(binding, vec![0,1,2])
    }
}
