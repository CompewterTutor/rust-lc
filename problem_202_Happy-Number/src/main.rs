// Write an algorithm to determine if a number n is happy.

// A happy number is a number defined by the following process:

//     Starting with any positive integer, replace the number by the sum of the squares of its digits.
//     Repeat the process until the number equals 1 (where it will stay), or it loops endlessly 
//     in a cycle which does not include 1.
//     Those numbers for which this process ends in 1 are happy.

// Return true if n is a happy number, and false if not.

 

// Example 1:

// Input: n = 19
// Output: true
// Explanation:
// 12 + 92 = 82
// 82 + 22 = 68
// 62 + 82 = 100
// 12 + 02 + 02 = 1

// Example 2:

// Input: n = 2
// Output: false

 

// Constraints:

//     1 <= n <= 231 - 1
use std::collections::HashSet;

pub fn sum_of_squares(mut num:i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        let digit = num % 10;
        sum += digit * digit;
        num /= 10;
    }
    sum
}

pub fn is_happy(n: i32) -> bool {
    let mut seen = HashSet::new();
    let mut num = n;

    while num != 1 {
        if seen.contains(&num) {
            return false;
        }
        seen.insert(num);
        num = sum_of_squares(num);
    }
    true

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_happy_number() {
        assert_eq!(is_happy(19), true);
    }

    #[test]
    fn test_unhappy_number() {
        assert_eq!(is_happy(2), false);
    }

    #[test]
    fn test_happy_number_1() {
        assert_eq!(is_happy(1), true);
    }

    #[test]
    fn test_unhappy_number_3() {
        assert_eq!(is_happy(3), false);
    }
}

fn main() {
    println!("Hello, world!");
}
