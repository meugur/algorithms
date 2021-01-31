//!
//! Determines the minimum number of operations to transform two strings
//!
//! You can insert one char, delete one char, or change one char
//!

use std::cmp;

fn min_ops(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a == 0 {
        return len_b;
    }
    if len_b == 0 {
        return len_a;
    }
    let mut r = vec![vec![0; len_b+1]; len_a+1];
    for i in 0..len_a+1 {
        r[i][0] = i;
    }
    for i in 0..len_b+1 {
        r[0][i] = i;
    }
    for (i, c_a) in a.chars().enumerate() {
        for (j, c_b) in b.chars().enumerate() {
            let t = (c_a != c_b) as usize;
            r[i+1][j+1] = cmp::min(
                r[i][j] + t,
                cmp::min(
                    r[i][j+1] + 1,
                    r[i+1][j] + 1,
                ),
            );
        }
    }
    r[len_a][len_b]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        assert_eq!(min_ops("kitten", "sitting"), 3);
        assert_eq!(min_ops("hello", "money"), 5); 
        assert_eq!(min_ops("cat", "car"), 1);
    }

    #[test]
    fn test_empty() {
        assert_eq!(min_ops("", "test"), 4);
        assert_eq!(min_ops("test", ""), 4);
    }

    #[test]
    fn test_equal() {
        assert_eq!(min_ops("hey", "hey"), 0);
    }
}
