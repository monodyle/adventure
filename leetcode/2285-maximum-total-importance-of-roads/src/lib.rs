pub fn maximum_importance(n: i32, roads: Vec<Vec<i32>>) -> i64 {
    let mut graph = vec![0; n as usize];
    for road in roads {
        graph[road[0] as usize] += 1;
        graph[road[1] as usize] += 1;
    }

    graph.sort_unstable();

    let mut value: i64 = 1;
    let mut total: i64 = 0;
    for node in graph {
        total += value * node;
        value += 1;
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
