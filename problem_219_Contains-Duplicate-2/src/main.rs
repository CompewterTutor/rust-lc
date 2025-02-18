// Given an integer array nums and an integer k, return true if there are
// two distinct indices i and j in the array such that nums[i] == nums[j] and
// abs(i - j) <= k.


// Example 1:

// Input: nums = [1,2,3,1], k = 3
// Output: true

// Example 2:

// Input: nums = [1,0,1,1], k = 1
// Output: true

// Example 3:

// Input: nums = [1,2,3,1,2,3], k = 2
// Output: false

// Constraints:

//     1 <= nums.length <= 105
//     -109 <= nums[i] <= 109
//     0 <= k <= 105
use std::collections::HashMap;

pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
    let mut index_map = HashMap::new();

    for (i, &num) in nums.iter().enumerate() {
        if let Some(&prev_index) = index_map.get(&num) {
            if (i as i32 - prev_index) <= k {
                return true;
            }
        }
        index_map.insert(num, i as i32);
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let nums = vec![1, 2, 3, 1];
        let k = 3;
        assert_eq!(contains_nearby_duplicate(nums, k), true);
    }

    #[test]
    fn test_example_2() {
        let nums = vec![1, 0, 1, 1];
        let k = 1;
        assert_eq!(contains_nearby_duplicate(nums, k), true);
    }

    #[test]
    fn test_example_3() {
        let nums = vec![1, 2, 3, 1, 2, 3];
        let k = 2;
        assert_eq!(contains_nearby_duplicate(nums, k), false);
    }
}

fn main() {
    println!("Hello, world!");
}
