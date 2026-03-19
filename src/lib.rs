#![no_std]
mod vector;
mod matrix;
mod tensor;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use core::ops::{Add, Mul};
    use crate::vector::Vector;
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_add() {
        let vec1 = Vector::new([3.0, 4.0]);
        let vec2 = Vector::new([1.0, 2.0]);
        
        let vec3 = vec1 + vec2;
        assert_eq!(vec3, Vector::new([4.0, 6.0]));
    }

    #[test]
    fn test_mul_scalar() {
        let vec1 = Vector::new([3.0, 4.0]);
        let scalar = 3.0;
        
        assert_eq!(vec1 * scalar, Vector::new([9.0, 12.0]));
    }
}
