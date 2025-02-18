// Given a pattern and a string s, find if s follows the same pattern.

// Here follow means a full match, such that there is a bijection between a letter in pattern and a non-empty word in s. Specifically:

//     Each letter in pattern maps to exactly one unique word in s.
//     Each unique word in s maps to exactly one letter in pattern.
//     No two letters map to the same word, and no two words map to the same letter.

 

// Example 1:

// Input: pattern = "abba", s = "dog cat cat dog"

// Output: true

// Explanation:

// The bijection can be established as:

//     'a' maps to "dog".
//     'b' maps to "cat".

// Example 2:

// Input: pattern = "abba", s = "dog cat cat fish"

// Output: false

// Example 3:

// Input: pattern = "aaaa", s = "dog cat cat dog"

// Output: false

 

// Constraints:

//     1 <= pattern.length <= 300
//     pattern contains only lower-case English letters.
//     1 <= s.length <= 3000
//     s contains only lowercase English letters and spaces ' '.
//     s does not contain any leading or trailing spaces.
//     All the words in s are separated by a single space.

pub fn word_pattern(pattern: String, s:String) -> bool {
    let mut pattern_map = std::collections::HashMap::new();
    let mut word_map = std::collections::HashMap::new();
    let words: Vec<&str> = s.split_whitespace().collect();
    if pattern.len() != words.len() {
        return false;
    }
    for (i, c) in pattern.chars().enumerate() {
        if pattern_map.contains_key(&c) {
            if pattern_map[&c] != words[i] {
                return false;
            }
        } else {
            pattern_map.insert(c, words[i]);
        }
        if word_map.contains_key(words[i]) {
            if word_map[words[i]] != c {
                return false;
            }
        } else {
            word_map.insert(words[i], c);
        }
    }
    true
}

fn word_pattern_zip(pattern: String, s: String) -> bool {
    let words: Vec<&str> = s.split_whitespace().collect();
    let pattern_chars: Vec<char> = pattern.chars().collect();

    if words.len() != pattern_chars.len() {
        return false;
    }

    let mut char_to_word: HashMap<char, &str> = HashMap::new();
    let mut word_to_char: HashMap<&str, char> = HashMap::new();

    for (ch, word) in pattern_chars.iter().zip(words.iter()) {
        if let Some(&mapped_word) = char_to_word.get(ch) {
            if mapped_word != *word {
                return false;
            }
        } else {
            char_to_word.insert(*ch, *word);
        }

        if let Some(&mapped_char) = word_to_char.get(word) {
            if mapped_char != *ch {
                return false;
            }
        } else {
            word_to_char.insert(*word, *ch);
        }
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_word_pattern() {
        assert_eq!(word_pattern("abba".to_string(), "dog cat cat dog".to_string()), true);
        assert_eq!(word_pattern("abba".to_string(), "dog cat cat fish".to_string()), false);
        assert_eq!(word_pattern("aaaa".to_string(), "dog cat cat dog".to_string()), false);
        assert_eq!(word_pattern("abba".to_string(), "dog dog dog dog".to_string()), false);
        assert_eq!(word_pattern("abc".to_string(), "dog cat fish".to_string()), true);
    }
}

fn main() {
    println!("Hello, world!");
}
