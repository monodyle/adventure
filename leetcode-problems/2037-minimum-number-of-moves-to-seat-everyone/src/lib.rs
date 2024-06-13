pub fn min_moves_to_seat(mut seats: Vec<i32>, mut students: Vec<i32>) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();

    seats.iter().zip(students.iter()).map(|(x,y)| (x - y).abs()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = min_moves_to_seat(vec![3,1,5], vec![2,7,4]);
        assert_eq!(result, 4);
    }

    #[test]
    fn example_2() {
        let result = min_moves_to_seat(vec![4,1,5,9], vec![1,3,2,6]);
        assert_eq!(result, 7);
    }

    #[test]
    fn example_3() {
        let result = min_moves_to_seat(vec![2,2,6,6], vec![1,3,2,6]);
        assert_eq!(result, 4);
    }
}
