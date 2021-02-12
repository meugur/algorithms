//!
//! Determines the largest sized rectangle in a histogram
//! 

fn largest_rect(a: &[usize]) -> usize {
    let len = a.len();
    if len == 0 {
        return 0;
    }
    if len == 1 {
        return a[0];
    }
    let mut stack: Vec<usize> = Vec::new();
    let mut max_area: usize = 0;

    let mut i = 0;
    while i < len {
        match stack.last() {
            None => {
                stack.push(i);
                i += 1;
                continue;
            },
            Some(&x) if a[x] <= a[i] => {
                stack.push(i);
                i += 1;
                continue;
            },
            Some(_) => {
                let top = stack.pop().unwrap();

                let h = a[top];
                let w =
                    match stack.last() {
                        None => i,
                        Some(&x) => i - x - 1,
                    };
                let area = w * h;
                if area > max_area {
                    max_area = area;
                }
            }
        }
    }
    while !stack.is_empty() {
        let top = stack.pop().unwrap();

        let h = a[top];
        let w =
            match stack.last() {
                None => i,
                Some(&x) => i - x - 1,
            };
        let area = w * h;
        if area > max_area {
            max_area = area;
        }
    }
    max_area
}


#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty() {
        assert_eq!(largest_rect(&[]), 0);
    }

    #[test]
    fn test_single() {
        assert_eq!(largest_rect(&[5]), 5);
    }

    #[test]
    fn test_basic() {
        assert_eq!(largest_rect(&[1, 2, 3, 1]), 4);
        assert_eq!(largest_rect(&[3, 1, 1, 1]), 4);
        assert_eq!(largest_rect(&[2, 5, 1, 3]), 5);
        assert_eq!(largest_rect(&[1, 2, 3, 4]), 6);
        assert_eq!(largest_rect(&[2, 2, 2, 2]), 8);
    }
}
