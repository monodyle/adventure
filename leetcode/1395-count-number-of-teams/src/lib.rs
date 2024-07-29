pub fn num_teams(rating: Vec<i32>) -> i32 {
    let mut result = 0;
    for i in 0..rating.len() {
        for j in i+1..rating.len() {
            for k in j+1..rating.len() {
                if (rating[i] < rating[j] && rating[j] < rating[k]) || (rating[i] > rating[j] && rating[j] > rating[k]) {
                    result += 1
                }
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = num_teams(vec![2,5,3,4,1]);
        assert_eq!(result, 3);
    }

    #[test]
    fn example_2() {
        let result = num_teams(vec![2,1,3]);
        assert_eq!(result, 0);
    }

    #[test]
    fn example_3() {
        let result = num_teams(vec![1,2,3,4]);
        assert_eq!(result, 4);
    }
}
