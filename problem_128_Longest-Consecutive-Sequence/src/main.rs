// Given an unsorted array of integers nums, return the length of the 
// longest consecutive elements sequence.

// You must write an algorithm that runs in O(n) time.

 

// Example 1:

// Input: nums = [100,4,200,1,3,2]
// Output: 4
// Explanation: The longest consecutive elements sequence is [1, 2, 3, 4]. Therefore its length is 4.

// Example 2:

// Input: nums = [0,3,7,2,5,8,4,6,0,1]
// Output: 9

// Example 3:

// Input: nums = [1,0,1,2]
// Output: 3

 

// Constraints:

//     0 <= nums.length <= 105
//     -109 <= nums[i] <= 109


use std::collections::HashSet;
pub fn longest_consecutive(nums: Vec<i32>) -> i32 {
    let num_set: HashSet<i32> = nums.into_iter().collect();
    let mut longest_streak = 0;

    for &num in &num_set {
        if !num_set.contains(&(num - 1)) {
            let mut current_num = num;
            let mut current_streak = 1;

            while num_set.contains(&(current_num + 1)) {
                current_num += 1;
                current_streak += 1;
            }

            longest_streak = longest_streak.max(current_streak);
        }
    }

    longest_streak
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![100, 4, 200, 1, 3, 2];
        assert_eq!(longest_consecutive(nums), 4);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![0, 3, 7, 2, 5, 8, 4, 6, 0, 1];
        assert_eq!(longest_consecutive(nums), 9);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 0, 1, 2];
        assert_eq!(longest_consecutive(nums), 3);
    }

    #[test]
    fn test_empty() {
        let nums: Vec<i32> = vec![];
        assert_eq!(longest_consecutive(nums), 0);
    }

    #[test]
    fn test_single_element() {
        let nums = vec![1];
        assert_eq!(longest_consecutive(nums), 1);
    }
}

fn main() {
    println!("Hello, world!");
}
