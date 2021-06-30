#![allow(dead_code)]

use std::cmp::Ordering;

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

fn search<T: Ord>(key: T, elems: &[T]) -> Option<usize> {
    let mut low = 0;
    let mut high = elems.len();

    while low != high {
        let m = (high - low) / 2 + low;

        match elems[m].cmp(&key) {
            Ordering::Equal => return Some(m),
            Ordering::Less => {
                if m == elems.len() - 1 {
                    return None;
                }

                low = m + 1;
            }
            Ordering::Greater => {
                if m == 0 {
                    return None;
                }

                high = m - 1;
            }
        }
    }

    if elems[low] == key {
        Some(low)
    } else {
        None
    }
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
    fn test_search() {
        let a = [1, 3, 6, 8, 12, 20];
        for i in 0..a.len() {
            assert_eq!(search(a[i], &a), Some(i));
        }

        assert_eq!(search(0, &a), None);
        assert_eq!(search(21, &a), None);
        assert_eq!(search(2, &a), None);
        assert_eq!(search(9, &a), None);

        let a = [1, 3, 6, 8, 12];
        for i in 0..a.len() {
            assert_eq!(search(a[i], &a), Some(i));
        }

        assert_eq!(search(0, &a), None);
        assert_eq!(search(21, &a), None);
        assert_eq!(search(2, &a), None);
        assert_eq!(search(9, &a), None);
    }

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
