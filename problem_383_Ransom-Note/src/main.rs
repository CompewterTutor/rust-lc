// Given two strings ransomNote and magazine, return true if ransomNote can be constructed by using the letters from magazine and false otherwise.

// Each letter in magazine can only be used once in ransomNote.

 

// Example 1:

// Input: ransomNote = "a", magazine = "b"
// Output: false

// Example 2:

// Input: ransomNote = "aa", magazine = "ab"
// Output: false

// Example 3:

// Input: ransomNote = "aa", magazine = "aab"
// Output: true

 

// Constraints:

//     1 <= ransomNote.length, magazine.length <= 105
//     ransomNote and magazine consist of lowercase English letters.

use std::collections::HashMap;

fn can_construct(ransom_note: String, magazine: String) -> bool {
    let mut letter_count = HashMap::new();

    // Count the frequency of each character in magazine
    for ch in magazine.chars() {
        *letter_count.entry(ch).or_insert(0) += 1;
    }

    // Check if ransom_note can be formed
    for ch in ransom_note.chars() {
        let count = letter_count.entry(ch).or_insert(0);
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
