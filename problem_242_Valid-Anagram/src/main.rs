// Given two strings s and t, return true if t is an
// anagram
// of s, and false otherwise.

 

// Example 1:

// Input: s = "anagram", t = "nagaram"

// Output: true

// Example 2:

// Input: s = "rat", t = "car"

// Output: false

 

// Constraints:

//     1 <= s.length, t.length <= 5 * 104
//     s and t consist of lowercase English letters.

 

// Follow up: What if the inputs contain Unicode characters? How would you adapt your solution to such a case?

pub fn is_anagram(s: String, t: String) -> bool {
    //check for equal lengths
    if s.len() != t.len() {
        return false;
    }

    let mut count_map = HashMap::new();

    for ch in s.chars() {
        *count_map.entry(ch).or_insert(0) += 1;
    }

    for ch in t.chars() {
        let count = count_map.entry(ch).or_insert(0);
        if *count == 0 {
            return false;
        }
        *count -= 1;
    }

    true
}

fn main() {
    println!("Hello, world!");
}
