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

pub fn sort_colors_pointer(nums: &mut Vec<i32>) {
    /*
        0s => [0,i0)
        1s => [i0,i2)
        2s => [i2,len(nums))
     */
    let (mut i, mut i0, mut i2) = (0, 0, nums.len());

    while i < i2 {
        match nums[i] {
            0 => {
                nums.swap(i, i0);
                i0 += 1;
                i += 1;
            },
            2 => {
                nums.swap(i, i2-1);
                i2 -= 1;
            },
            _ => i += 1
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

    #[test]
    fn example_2_1() {
        let mut binding = vec![2,0,2,1,1,0];
        sort_colors_pointer(&mut binding);
        assert_eq!(binding, vec![0,0,1,1,2,2]);
    }

    #[test]
    fn example_2_2() {
        let mut binding = vec![2,0,1];
        sort_colors_pointer(&mut binding);
        assert_eq!(binding, vec![0,1,2]);
    }
}
