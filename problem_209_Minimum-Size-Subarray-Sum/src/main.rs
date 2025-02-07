// Given an array of positive integers nums and a positive integer target, 
// return the minimal length of a subarray whose sum is greater than or equal to target. 
// If there is no such subarray, return 0 instead.

 

// Example 1:

// Input: target = 7, nums = [2,3,1,2,4,3]
// Output: 2
// Explanation: The subarray [4,3] has the minimal length under the problem constraint.

// Example 2:

// Input: target = 4, nums = [1,4,4]
// Output: 1

// Example 3:

// Input: target = 11, nums = [1,1,1,1,1,1,1,1]
// Output: 0

 

// Constraints:

//     1 <= target <= 109
//     1 <= nums.length <= 105
//     1 <= nums[i] <= 104

 
// Follow up: If you have figured out the O(n) solution, try coding another solution of which the time complexity is O(n log(n)).


pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        

    return 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(min_sub_array_len(7, vec![2, 3, 1, 2, 4, 3]), 2);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(min_sub_array_len(4, vec![1, 4, 4]), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(min_sub_array_len(11, vec![1, 1, 1, 1, 1, 1, 1, 1]), 0);
    }

    #[test]
    fn test_empty_array() {
        assert_eq!(min_sub_array_len(5, vec![]), 0);
    }

    #[test]
    fn test_single_element() {
        assert_eq!(min_sub_array_len(5, vec![5]), 1);
        assert_eq!(min_sub_array_len(5, vec![4]), 0);
    }
}

fn main() {
    println!("Hello, world!");
}
