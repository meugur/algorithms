pub struct Point<T>
{
    pub x: T,
    pub y: T,
}

impl Point<isize> {
    pub fn distance_to(&self, p: &Point<isize>) -> f32 {
        f32::sqrt(
            (self.x - p.x).pow(2) as f32 +
            (self.y - p.y).pow(2) as f32
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zero_distance() {
        let a = Point{ x: 2, y: 1 };
        let b = Point{ x: 2, y: 1 }; 
        assert_eq!(a.distance_to(&b), 0.0);
    }

    #[test]
    fn test_basic_distance() {
        let a = Point{ x: 3, y: 1 };
        let b = Point{ x: 1, y: 1 }; 
        assert_eq!(a.distance_to(&b), 2.0);
    }
}
