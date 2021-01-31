//!
//! Finds the minimum distance between many points
//!
//! TODO: Divide-and-conquer algorithm for O(nlogn) solution
//!

use crate::point::Point;

fn brute_force(points: &Vec<Point<isize>>) -> f32 {
    if points.len() == 0 || points.len() == 1 {
        return 0.0;
    }
    let mut min = std::f32::MAX;
    for i in 0..points.len() {
        for j in (i+1)..points.len() {
            let d = points[i].distance_to(&points[j]);
            if d < min {
                min = d;
            }
        }
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let v = vec![
            Point{ x: 1, y: 2 },
            Point{ x: 3, y: 2 },
        ];
        assert_eq!(brute_force(&v), 2.0);
    }

    #[test]
    fn test_empty() {
        let zero = vec![];
        assert_eq!(brute_force(&zero), 0.0);

        let one = vec![Point{ x: 1, y: 2 }];
        assert_eq!(brute_force(&one), 0.0);
    }
}
