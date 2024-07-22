pub fn sort_people(names: Vec<String>, heights: Vec<i32>) -> Vec<String> {
    let mut ans = names.into_iter().zip(heights.into_iter()).collect::<Vec<_>>();
    ans.sort_unstable_by_key(|p| -p.1);
    ans.into_iter().map(|p| p.0).collect()
}
