pub fn longest_common_prefix(strs: Vec<String>) -> String {
    // If the input vector is empty, return an empty string
    if strs.is_empty() {
        return "".to_string();
    }
    
    // Initialize the prefix with the first string in the vector
    let mut prefix = strs[0].clone();
    
    // Iterate over the rest of the strings in the vector
    for i in 1..strs.len() {
        let mut j = 0;
        let mut new_prefix = String::new();
        
        // Compare characters of the current string with the prefix
        for c in strs[i].chars() {
            // If characters match, add to new_prefix
            if j < prefix.len() && c == prefix.chars().nth(j).unwrap() {
                new_prefix.push(c);
                j += 1;
            } else {
                // Break if characters do not match
                break;
            }
        }
        
        // Update the prefix with the new common prefix found
        prefix = new_prefix;
    }
    
    // Return the longest common prefix
    prefix
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        assert_eq!(longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]), "fl".to_string());
        assert_eq!(longest_common_prefix(vec!["dog".to_string(), "racecar".to_string(), "car".to_string()]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["interspecies".to_string(), "interstellar".to_string(), "interstate".to_string()]), "inters".to_string());
        assert_eq!(longest_common_prefix(vec!["throne".to_string(), "throne".to_string(), "throne".to_string()]), "throne".to_string());
        assert_eq!(longest_common_prefix(vec!["".to_string()]), "".to_string());
        assert_eq!(longest_common_prefix(vec!["a".to_string()]), "a".to_string());
        assert_eq!(longest_common_prefix(vec!["".to_string(), "".to_string()]), "".to_string());
    }
}
