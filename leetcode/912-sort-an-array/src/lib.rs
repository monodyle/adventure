pub fn sort_array(nums: Vec<i32>) -> Vec<i32> {
    if nums.len() < 2 {
        return nums.to_vec()
    }

    let size = nums.len() / 2;
    let left = sort_array(nums[0..size].to_vec());
    let right = sort_array(nums[size..].to_vec());

    merge(&left, &right)
}

fn merge(left: &Vec<i32>, right: &Vec<i32>) -> Vec<i32> {
    let mut i = 0;
    let mut j = 0;
    let mut merged: Vec<i32> = Vec::new();

    while i < left.len() && j < right.len() {
        if left[i] < right[j] {
            merged.push(left[i]);
            i += 1;
        } else {
            merged.push(right[j]);
            j += 1;
        }
    }

    if i < left.len() {
        while i < left.len() {
            merged.push(left[i]);
            i += 1;
        }
    }

    if j < right.len() {
        while j < right.len() {
            merged.push(right[j]);
            j += 1;
        }
    }

    merged
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = sort_array(vec![5,2,3,1]);
        assert_eq!(result, vec![1,2,3,5]);
    }

    #[test]
    fn example_2() {
        let result = sort_array(vec![5,1,1,2,0,0]);
        assert_eq!(result, vec![0,0,1,1,2,5]);
    }
}
