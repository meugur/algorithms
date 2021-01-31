//!
//! Determines the best price to sell a rod of size n when cut
//! 

use std::cmp;

fn bottom_up(n: usize, p: &[isize]) -> isize {
    if n == 0 {
        return 0;
    }
    let mut r = vec![std::isize::MIN; n+1];
    r[0] = 0;

    for i in 1..n+1 {
        for j in 1..i+1 {
            r[i] = cmp::max(r[i], p[j-1] + r[i-j]);
        }
    }
    return r[n];
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic() {
        let n = 1;
        let prices = &[1];
        let r = bottom_up(n, prices);
        assert_eq!(r, 1);
    }

    #[test]
    fn test_empty() {
        let n = 0;
        let prices = &[];
        let r = bottom_up(n, prices);
        assert_eq!(r, 0);
    }

    #[test]
    fn test_correctness_1() {
        let n = 2;
        let prices = &[2, 2];
        let r = bottom_up(n, prices);
        assert_eq!(r, 4);
    }

    #[test]
    fn test_correctness_2() {
        let n = 4;
        let prices = &[1, 5, 8, 9];
        let r = bottom_up(n, prices);
        assert_eq!(r, 10);
    }
}
