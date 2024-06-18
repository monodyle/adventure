pub fn find_order(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> Vec<i32> {
    let n = num_courses as usize;
    let mut ans: Vec<i32> = vec![];
    let mut visited = vec![false; n];
    let mut courses = vec![0; n];
    let mut graph = vec![vec![]; n];

    for k in 0..prerequisites.len() {
        let i = prerequisites[k][0] as usize;
        let j = prerequisites[k][1] as usize;

        graph[j].push(i);
        courses[i] += 1;
    }

    let mut queue = Vec::new();

    for i in 0..n {
        if courses[i] == 0 {
            queue.push(i);
            visited[i] = true;
        }
    }

    while !queue.is_empty() {
        let i = queue.pop().unwrap();
        ans.push(i as i32);
        for &j in &graph[i] {
            if !visited[j] {
                courses[j] -= 1;

                if courses[j] == 0 {
                    queue.push(j);
                    visited[j] = true;
                }
            }
        }
    }

    if ans.len() == n {
        ans
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = find_order(2, vec![vec![1, 0]]);
        assert_eq!(result, vec![0, 1]);
    }

    #[test]
    fn example_2() {
        let result = find_order(4, vec![vec![1, 0], vec![2, 0], vec![3, 1], vec![3, 2]]);
        assert_eq!(result, vec![0, 2, 1, 3]);
    }

    #[test]
    fn example_3() {
        let result = find_order(1, vec![]);
        assert_eq!(result, vec![0]);
    }
}
