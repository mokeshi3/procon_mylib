#![allow(dead_code)]
fn gcd(a: usize, b: usize) -> usize {
    let a = a % b;

    if a == 0 {
        return b;
    }

    let greater = a.max(b);
    let smaller = a.min(b);

    gcd(greater, smaller)
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 6), 2);
        assert_eq!(gcd(12, 16), 4);
        assert_eq!(gcd(16, 12), 4);
        assert_eq!(gcd(123, 456), 3);
    }

    #[test]
    fn test_lcm() {
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(2, 4), 4);
        assert_eq!(lcm(12, 10), 60);
        assert_eq!(lcm(6, 7), 42);
        assert_eq!(lcm(123, 456), 18696);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
