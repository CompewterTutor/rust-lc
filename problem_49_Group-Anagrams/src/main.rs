use std::collections::HashMap;
pub fn group_anagrams(strs: Vec<String>) -> Vec<Vec<String>> {
    let mut map: HashMap<String, Vec<String>> = HashMap::new();
    for s in strs {
        let mut chars: Vec<char> = s.chars().collect();
        chars.sort_unstable();
        let sorted_str = chars.iter().collect::<String>();
        map.entry(sorted_str).or_insert(Vec::new()).push(s);
    }


    map.into_values().collect()
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_group_anagrams() {
        let input = vec!["eat".to_string(), "tea".to_string(), "tan".to_string(), "ate".to_string(), "nat".to_string(), "bat".to_string()];
        let mut result = group_anagrams(input);
        for group in &mut result {
            group.sort();
        }
        result.sort();
        let expected = vec![
            vec!["ate".to_string(), "eat".to_string(), "tea".to_string()],
            vec!["bat".to_string()],
            vec!["nat".to_string(), "tan".to_string()],
        ];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_empty_input() {
        let input: Vec<String> = vec![];
        let result = group_anagrams(input);
        let expected: Vec<Vec<String>> = vec![];
        assert_eq!(result, expected);
    }

    #[test]
    fn test_single_word() {
        let input = vec!["hello".to_string()];
        let result = group_anagrams(input);
        let expected = vec![vec!["hello".to_string()]];
        assert_eq!(result, expected);
    }
}
