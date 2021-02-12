//!
//! Determines the longest common subsequence between two strings
//! 

fn longest_common_subseq(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();

    if len_a == 0 || len_b == 0{
        return 0;
    }
    let mut r = vec![vec![0; len_b+1]; len_a+1];

    for (i, c_a) in a.chars().enumerate() {
        for (j, c_b) in b.chars().enumerate() {
            if c_a == c_b {
                r[i+1][j+1] = r[i][j] + 1;
            } else {
                r[i+1][j+1] = std::cmp::max(r[i][j+1], r[i+1][j]);
            }
        }
    }
    r[len_a][len_b]
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        assert_eq!(longest_common_subseq("", "abc"), 0);
        assert_eq!(longest_common_subseq("abc", ""), 0);
    }

    #[test]
    fn test_single() {
        assert_eq!(longest_common_subseq("a", "a"), 1);
        assert_eq!(longest_common_subseq("a", "b"), 0);
    }

    #[test]
    fn test_basic() {
        assert_eq!(
            longest_common_subseq(
                "werqkjl",
                "ewrkjl"
            ),
            5
        );
    }
}
