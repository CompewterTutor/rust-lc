// You are given a string s and an array of strings words. All the strings of words are of the same length.

// A concatenated string is a string that exactly contains all the strings of any permutation of words concatenated.

//     For example, if words = ["ab","cd","ef"], then "abcdef", "abefcd", "cdabef", "cdefab", "efabcd", and "efcdab" 
//      are all concatenated strings. "acdbef" is not a concatenated string because it is not the concatenation of any 
//      permutation of words.

// Return an array of the starting indices of all the concatenated substrings in s. You can return the answer in any order.


// Example 1:

// Input: s = "barfoothefoobarman", words = ["foo","bar"]

// Output: [0,9]

// Explanation:

// The substring starting at 0 is "barfoo". It is the concatenation of ["bar","foo"] which is a permutation of words.
// The substring starting at 9 is "foobar". It is the concatenation of ["foo","bar"] which is a permutation of words.

// Example 2:

// Input: s = "wordgoodgoodgoodbestword", words = ["word","good","best","word"]

// Output: []

// Explanation:

// There is no concatenated substring.

// Example 3:

// Input: s = "barfoofoobarthefoobarman", words = ["bar","foo","the"]

// Output: [6,9,12]

// Explanation:

// The substring starting at 6 is "foobarthe". It is the concatenation of ["foo","bar","the"].
// The substring starting at 9 is "barthefoo". It is the concatenation of ["bar","the","foo"].
// The substring starting at 12 is "thefoobar". It is the concatenation of ["the","foo","bar"].

 
// Constraints:

//     1 <= s.length <= 104
//     1 <= words.length <= 5000
//     1 <= words[i].length <= 30
//     s and words[i] consist of lowercase English letters.

use std::collections::HashMap;

pub fn find_substring(s: String, words: Vec<String>) -> Vec<i32> {
    let word_count = words.len();
    let mut result = Vec::new();
    if word_count == 0 {
        return result;
    }
    
    let word_len = words[0].len();
    
    let concat_len = word_len * word_count;
    

    if s.len() < concat_len {
        return result;
    }

    let mut word_map = HashMap::new();
    for word in &words {
        *word_map.entry(word.clone()).or_insert(0) += 1;
    }

    for i in 0..word_len {
        let mut left = i;
        let mut right = i;
        let mut window_map = HashMap::new();
        let mut count = 0;

        while right + word_len <= s.len() {
            let word = &s[right..right + word_len];

            if word_map.contains_key(word) {
                *window_map.entry(word.to_string()).or_insert(0) += 1;
                count += 1;
                right += word_len;

                while window_map[&word.to_string()] > word_map[word] {
                    let left_word = &s[left..left + word_len];
                    *window_map.get_mut(left_word).unwrap() -= 1;
                    count -= 1;
                    left += word_len;
                }

                if count == word_count {
                    result.push(left as i32);
                }
            } else {
                window_map.clear();
                count = 0;
                right += word_len;
                left = right;
            }
        }
    }

    result


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_1() {
        let s = "barfoothefoobarman".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = find_substring(s, words);
        assert_eq!(result, vec![0, 9]);
    }

    #[test]
    fn test_example_2() {
        let s = "wordgoodgoodgoodbestword".to_string();
        let words = vec!["word".to_string(), "good".to_string(), "best".to_string(), "word".to_string()];
        let result = find_substring(s, words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_example_3() {
        let s = "barfoofoobarthefoobarman".to_string();
        let words = vec!["bar".to_string(), "foo".to_string(), "the".to_string()];
        let result = find_substring(s, words);
        assert_eq!(result, vec![6, 9, 12]);
    }

    #[test]
    fn test_empty_string() {
        let s = "".to_string();
        let words = vec!["foo".to_string(), "bar".to_string()];
        let result = find_substring(s, words);
        assert_eq!(result, vec![]);
    }

    #[test]
    fn test_empty_words() {
        let s = "barfoothefoobarman".to_string();
        let words: Vec<String> = vec![];
        let result = find_substring(s, words);
        assert_eq!(result, vec![]);
    }
}

fn main() {
    println!("Hello, world!");
}
