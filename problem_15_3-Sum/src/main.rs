pub fn three_sum(mut nums: Vec<i32>) -> Vec<Vec<i32>> {
    nums.sort(); // Sort the input array
    let mut result = Vec::new();
    
    let n = nums.len();
    
    for i in 0..n {
        if i > 0 && nums[i] == nums[i - 1] {
            continue; // Skip duplicates for the first element
        }

        let (mut left, mut right) = (i + 1, n - 1);
        
        while left < right {
            let sum = nums[i] + nums[left] + nums[right];

            if sum == 0 {
                result.push(vec![nums[i], nums[left], nums[right]]);
                
                // Skip duplicates for the second element
                while left < right && nums[left] == nums[left + 1] {
                    left += 1;
                }

                // Skip duplicates for the third element
                while left < right && nums[right] == nums[right - 1] {
                    right -= 1;
                }

                left += 1;
                right -= 1;
            } else if sum < 0 {
                left += 1;
            } else {
                right -= 1;
            }
        }
    }
    
    result
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_three_sum() {
        let nums = vec![-1, 0, 1, 2, -1, -4];
        let mut result = three_sum(nums);
        result.sort();
        let mut expected = vec![vec![-1, -1, 2], vec![-1, 0, 1]];
        expected.sort();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_sum_no_result() {
        let nums = vec![1, 2, -2, -1];
        let result = three_sum(nums);
        let expected: Vec<Vec<i32>> = Vec::new();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_three_sum_all_zeros() {
        let nums = vec![0, 0, 0, 0];
        let result = three_sum(nums);
        let expected = vec![vec![0, 0, 0]];
        assert_eq!(result, expected);
    }
}
