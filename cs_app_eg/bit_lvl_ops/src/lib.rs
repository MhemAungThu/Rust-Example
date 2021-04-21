// practice problem 2.13
pub fn bits(x: i32, m: i32) -> i32 {
    x  | m
}

pub fn bitc(x: i32, m: i32) -> i32 {
    (x ^ m) & !m
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bits() {
        let x = 118;
        let m = 10;

        assert_eq!(bits(x, m), 126);
    }
}
