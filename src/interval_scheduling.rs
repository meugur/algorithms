//!
//! Given the start and finish times of n times,
//! determines the maximum number of intervals that can be scheduled
//! 

fn interval_scheduling(intervals: &mut Vec<(usize, usize)>) -> usize {
    if intervals.is_empty() {
        return 0
    }
    if intervals.len() == 1 {
        return 1
    }
    intervals.sort_by_key(|t| t.1);

    let mut total = 0;
    let mut cur_finish = 0;
    for i in 0..intervals.len() {
        if cur_finish <= intervals[i].0 {
            total += 1;
            cur_finish = intervals[i].1
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test] 
    fn test_empty() {
        assert_eq!(interval_scheduling(&mut vec![]), 0);
    }

    #[test] 
    fn test_single() {
        assert_eq!(interval_scheduling(&mut vec![(1, 2)]), 1);
    }

    #[test] 
    fn test_double() {
        assert_eq!(
            interval_scheduling(
                &mut vec![
                    (2, 3),
                    (1, 2)
                ]
            ),
            2
        );
    }

    #[test] 
    fn test_many() {
        assert_eq!(
            interval_scheduling(
                &mut vec![
                    (3, 4),
                    (2, 5),
                    (2, 3),
                    (1, 2),
                    (2, 4),
                    (3, 8),
                    (3, 4),
                    (6, 7),
                    (5, 6),
                    (4, 5),
                    (3, 5),
                    (4, 6),
                ]
            ),
            6
        );
    }
}
