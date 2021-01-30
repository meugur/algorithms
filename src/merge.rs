pub fn sort<T: PartialOrd + Copy>(a: &mut [T]) {
    let len = a.len();
    if len <= 1 {
        return;
    }

    let mid = len / 2;
    sort(&mut a[..mid]);
    sort(&mut a[mid..]);

    let mut temp = Vec::with_capacity(len);
    let mut i = 0;
    let mut j = mid;

    while i < mid && j < len {
        if a[i] < a[j] {
            temp.push(a[i]);
            i += 1;
        } else {
            temp.push(a[j]);
            j += 1;
        }
    }
    while i < mid {
        temp.push(a[i]);
        i += 1;
    }
    while j < len {
        temp.push(a[j]);
        j += 1;
    }
    a.copy_from_slice(&temp[..]);
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
    fn sort_large() {
        let mut a: Vec<_> = (0..100000).rev().collect();
        let b: Vec<_> = (0..100000).collect();

        sort(&mut a);
        assert_eq!(a, b);
    }

    #[test]
    fn sort_even() {
        let a = &mut[4, 3, 2, 1];
        sort(a);
        assert_eq!(a, &[1, 2, 3, 4]);
    }
}
