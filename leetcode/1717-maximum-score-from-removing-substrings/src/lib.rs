pub fn maximum_gain(s: String, x: i32, y: i32) -> i32 {
    fn remove(s: &mut Vec<u8>, r: &[u8], x: i32) -> i32 {
        let mut pts = 0;
        let mut i = 0;
        for j in 0..s.len() {
            s[i] = s[j];
            i += 1;
            if i > 1 && s[i - 2] == r[0] && s[i - 1] == r[1] {
                i -= 2;
                pts += x;
            }
        }
        s.resize(i, 0);
        pts
    }

    let (mut x, mut y) = (x, y);
    let mut s = s.as_bytes().to_vec();
    let (mut a, mut b) = ("ab".as_bytes(), "ba".as_bytes());

    if x < y {
        std::mem::swap(&mut a, &mut b);
        std::mem::swap(&mut x, &mut y);
    }

    remove(&mut s, a, x) + remove(&mut s, b, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let result = maximum_gain("cdbcbbaaabab".to_string(), 4, 5);
        assert_eq!(result, 19);
    }

    #[test]
    fn example_2() {
        let result = maximum_gain("aabbaaxybbaabb".to_string(), 5, 4);
        assert_eq!(result, 20);
    }
}
