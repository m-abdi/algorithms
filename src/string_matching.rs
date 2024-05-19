//! ## string_matching
//!
//! This module provides different algorithms for string matching problems, which you have a text string and a pattern string and want to know all valid shifts of the pattern in the text. a valid shift like `s` is an index that shows pattern starts at `s+1`.

/// Finds all valid shifts with naive approach(sliding window of template). Time complexity: O((n - m + 1)m)
///
/// # Examples
///
/// ```
/// use algorithms::string_matching::naive_string_matcher;
/// let result = naive_string_matcher("aabaaa".to_string(), "aba".to_string());
/// assert_eq!(result, [0]);
/// ```
pub fn naive_string_matcher(text: String, pattern: String) -> Vec<usize> {
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    let mut valid_shifts: Vec<usize> = [].to_vec();
    'text_loop: for shift in 0..(text.len() - pattern.len() + 1) {
        for i in 0..pattern.len() {
            if pattern[i] != text[shift + i] {
                continue 'text_loop;
            } else if i == pattern.len() - 1 {
                valid_shifts.push(shift - 1);
            }
        }
    }
    valid_shifts
}

/// Finds all valid shifts with naive approach(sliding window of template) which all characters in the pattern are different. Time complexity: O(n)
///
/// # Examples
///
/// ```
/// use algorithms::string_matching::naive_string_matcher_special_pattern;
/// let result = naive_string_matcher_special_pattern("aabaaab".to_string(), "ab".to_string());
/// assert_eq!(result, [0, 4]);
/// ```
pub fn naive_string_matcher_special_pattern(text: String, pattern: String) -> Vec<usize> {
    let text: Vec<char> = text.chars().collect();
    let pattern: Vec<char> = pattern.chars().collect();
    let mut valid_shifts: Vec<usize> = [].to_vec();
    let mut ignored_characters = 0;
    'text_loop: for shift in 0..(text.len() - pattern.len() + 1) {
        if ignored_characters > 0 {
            ignored_characters -= 1;
            continue;
        }
        for i in 0..pattern.len() {
            if pattern[i] != text[shift + i] {
                continue 'text_loop;
            } else if i == pattern.len() - 1 {
                valid_shifts.push(shift - 1);
                ignored_characters += pattern.len() - 1;
            }
        }
    }
    valid_shifts
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_native_string_matcher() {
        let result = naive_string_matcher("aabaaa".to_string(), "aba".to_string());
        assert_eq!(result, [0]);
    }

    #[test]
    fn test_naive_string_matcher_special_pattern() {
        let result = naive_string_matcher_special_pattern("aabaaab".to_string(), "ab".to_string());
        assert_eq!(result, [0, 4]);
    }
}
