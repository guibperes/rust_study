use std::f64::consts::PI;

pub fn sum(x: i32, y: i32) -> i32 {
    x + y
}

pub fn area(r: f64) -> f64 {
    PI * r * r
}

#[cfg(test)]
mod test {
    use crate::math::sum;

    #[test]
    fn sum_test() {
        assert_eq!(sum(10, 10), 20);
    }
}
