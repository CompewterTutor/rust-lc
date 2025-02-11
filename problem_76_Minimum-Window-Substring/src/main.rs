// Given two strings s and t of lengths m and n respectively, return the minimum window
// substring of s such that every character in t (including duplicates) is included in the window. 
// If there is no such substring, return the empty string "".
// The testcases will be generated such that the answer is unique.

// Example 1:

// Input: s = "ADOBECODEBANC", t = "ABC"
// Output: "BANC"
// Explanation: The minimum window substring "BANC" includes 'A', 'B', and 'C' from string t.

// Example 2:

// Input: s = "a", t = "a"
// Output: "a"
// Explanation: The entire string s is the minimum window.

// Example 3:

// Input: s = "a", t = "aa"
// Output: ""
// Explanation: Both 'a's from t must be included in the window.
// Since the largest window of s only has one 'a', return empty string.

// Constraints:

//     m == s.length
//     n == t.length
//     1 <= m, n <= 105
//     s and t consist of uppercase and lowercase English letters.

// Follow up: Could you find an algorithm that runs in O(m + n) time?
use std::collections::HashMap;

pub fn min_window(s:String, t:String) -> String {
    let (mut left, mut right, mut start, mut min_len) = (0, 0, 0, usize::MAX);
    let mut required_chars = HashMap::new();
    let mut window_chars = HashMap::new();
    let mut formed = 0;

    // Populate the frequency map for `t`
    for c in t.chars() {
        *required_chars.entry(c).or_insert(0) += 1;
    }

    let required_len = required_chars.len();
    let s_chars: Vec<char> = s.chars().collect();

    while right < s.len() {
        let c = s_chars[right];
        *window_chars.entry(c).or_insert(0) += 1;

        if let Some(&count) = required_chars.get(&c) {
            if window_chars[&c] == count {
                formed += 1;
            }
        }

        while left <= right && formed == required_len {
            if right - left + 1 < min_len {
                min_len = right - left + 1;
                start = left;
            }

            let left_char = s_chars[left];
            *window_chars.entry(left_char).or_insert(0) -= 1;

            if let Some(&count) = required_chars.get(&left_char) {
                if window_chars[&left_char] < count {
                    formed -= 1;
                }
            }

            left += 1;
        }

        right += 1;
    }

    if min_len == usize::MAX {
        "".to_string()
    } else {
        s[start..start + min_len].to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_window() {
        assert_eq!(min_window("ADOBECODEBANC".to_string(), "ABC".to_string()), "BANC");
        assert_eq!(min_window("a".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("a".to_string(), "aa".to_string()), "");
        assert_eq!(min_window("a".to_string(), "b".to_string()), "");
        assert_eq!(min_window("ab".to_string(), "a".to_string()), "a");
        assert_eq!(min_window("ab".to_string(), "b".to_string()), "b");
        assert_eq!(min_window("ab".to_string(), "ab".to_string()), "ab");
    }
}

fn main() {
    println!("Hello, world!");
}
