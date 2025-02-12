// Given two strings s and t, determine if they are isomorphic.

// Two strings s and t are isomorphic if the characters in s can be replaced to get t.

// All occurrences of a character must be replaced with another character while preserving the order of characters. No two characters may map to the same character, but a character may map to itself.

// Example 1:

// Input: s = "egg", t = "add"

// Output: true

// Explanation:

// The strings s and t can be made identical by:

//     Mapping 'e' to 'a'.
//     Mapping 'g' to 'd'.

// Example 2:

// Input: s = "foo", t = "bar"

// Output: false

// Explanation:

// The strings s and t can not be made identical as 'o' needs to be mapped to both 'a' and 'r'.

// Example 3:

// Input: s = "paper", t = "title"

// Output: true

// Constraints:

//     1 <= s.length <= 5 * 104
//     t.length == s.length
//     s and t consist of any valid ascii character.
use std::collections::HashMap;

pub fn is_isomorphic(s: String, t: String) -> bool {
    if s.len() != t.len() {
        return false;
    }

    let mut s_to_t = HashMap::new();
    let mut t_to_s = HashMap::new();

    for (sc, tc) in s.chars().zip(t.chars()) {
        if let Some(&mapped) = s_to_t.get(&sc) {
            if mapped != tc {
                return false;
            }
        } else {
            s_to_t.insert(sc, tc);
        }

        if let Some(&mapped) = t_to_s.get(&tc) {
            if mapped != sc {
                return false;
            }
        } else {
            t_to_s.insert(tc, sc);
        }
    }

    true
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_isomorphic_strings() {
        assert!(is_isomorphic("egg".to_string(), "add".to_string()));
        assert!(!is_isomorphic("foo".to_string(), "bar".to_string()));
        assert!(is_isomorphic("paper".to_string(), "title".to_string()));
        assert!(!is_isomorphic("ab".to_string(), "aa".to_string()));
        assert!(is_isomorphic("".to_string(), "".to_string()));
    }
}
