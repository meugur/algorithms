pub fn sort<T: PartialOrd>(a: &mut [T]) {
    if a.len() == 0 {
        return;
    }
    for i in 0..(a.len() - 1) {
        for j in ((i + 1)..a.len()).rev() {
            if a[j] < a[j - 1] {
                a.swap(j, j - 1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_basic() {
        let a = &mut[5, 4, 3, 2, 1];
        sort(a);
        assert_eq!(a, &[1, 2, 3, 4, 5]);
    }

    #[test]
    fn sort_empty() {
        let a: &mut[u8] = &mut[];
        sort(a);
        assert_eq!(a, &[]);
    }

    #[test]
    fn sort_float() {
        let a = &mut[5.5, 4.4, 3.3, 2.2, 1.1];
        sort(a);
        assert_eq!(a, &[1.1, 2.2, 3.3, 4.4, 5.5]);
    }

    #[test]
    #[ignore]
    fn sort_large() {
        let mut a: Vec<_> = (0..10000).rev().collect();
        let b: Vec<_> = (0..10000).collect();

        sort(&mut a);
        assert_eq!(a, b);
    }
}
