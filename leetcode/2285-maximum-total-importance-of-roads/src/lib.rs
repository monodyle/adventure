pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut graph = roads.iter().fold(vec![0i64; n as usize], |mut acc, road| {
        acc[road[0] as usize] += 1;
        acc[road[1] as usize] += 1;
        acc
    });
    graph.sort_unstable();

    let mut total: i64 = 0;
    for i in 1..=n {
        total += graph[i as usize - 1] * i as i64;
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = maximum_importance(5, vec![vec![0,1],vec![1,2],vec![2,3],vec![0,2],vec![1,3],vec![2,4]]);
        assert_eq!(result, 43);
    }

    #[test]
    fn example_2() {
        let result = maximum_importance(5, vec![vec![0,3],vec![2,4],vec![1,3]]);
        assert_eq!(result, 20);
    }
}
