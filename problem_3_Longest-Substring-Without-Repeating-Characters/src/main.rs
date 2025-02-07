// Given a string s, find the length of the longest substring without repeating characters.
// Example 1:

// Input: s = "abcabcbb"
// Output: 3
// Explanation: The answer is "abc", with the length of 3.

// Example 2:

// Input: s = "bbbbb"
// Output: 1
// Explanation: The answer is "b", with the length of 1.

// Example 3:

// Input: s = "pwwkew"
// Output: 3
// Explanation: The answer is "wke", with the length of 3.
// Notice that the answer must be a substring, "pwke" is a subsequence and not a substring.

// Constraints:

//     0 <= s.length <= 5 * 104
//     s consists of English letters, digits, symbols and spaces.
use std::collections::HashSet;


pub fn length_of_longest_substring(s: String) -> i32 {
    // Implementation goes here
    let mut char_set = HashSet::new();
    let mut left = 0;
    let mut max_length = 0;
    let chars: Vec<char> = s.chars().collect();

    for right in 0..chars.len() {
        while char_set.contains(&chars[right]) {
            char_set.remove(&chars[left]);
            left += 1;
        }
        char_set.insert(chars[right]);
        max_length = max_length.max(right - left + 1);
    }

    max_length as i32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
    }

    #[test]
    fn test_example_2() {
        assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
    }

    #[test]
    fn test_example_3() {
        assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(length_of_longest_substring("".to_string()), 0);
    }

    #[test]
    fn test_single_character() {
        assert_eq!(length_of_longest_substring("a".to_string()), 1);
    }

    #[test]
    fn test_all_unique_characters() {
        assert_eq!(length_of_longest_substring("abcdef".to_string()), 6);
    }

    #[test]
    fn test_mixed_characters() {
        assert_eq!(length_of_longest_substring("aabcbdef".to_string()), 5);
    }
}

fn main() {
    println!("Hello, world!");
}
