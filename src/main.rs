mod chapter_1 {
    // Euclide's algorithm:
    // Find the greatest common divisor of two numbers
    // 1.1.E
    pub(crate) fn greatest_comomn_divisor(m: u64, n: u64) -> u64 {
        let remainder = m % n;
        if remainder == 0 {
            return n;
        } else {
            greatest_comomn_divisor(n, remainder)
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        #[test]
        fn test_greatest_common_divisor() {
            assert_eq!(17, greatest_comomn_divisor(119, 544));
        }
    }
}

fn main() {
    {
        println!("Chapter 1, algorithm E");
        let m = 119;
        let n = 544;
        println!(
            "The greatest common divisor of {} and {} is {}",
            m,
            n,
            chapter_1::greatest_comomn_divisor(m, n)
        );
    }
}
