// Added
struct Solution {}

// Solution
impl Solution {
    pub fn reverse_vowels(s: String) -> String {
        let vowels: Vec<char> = "aeiouAEIOU".chars().collect();
        let mut chars: Vec<char> = s.chars().collect();
        let mut left = 0;
        let mut right = chars.len() - 1;

        while left < right {
            while left < right && !vowels.contains(&chars[left]) {
                left += 1;
            }
            while left < right && !vowels.contains(&chars[right]) {
                right -= 1;
            }

            if left < right {
                chars.swap(left, right);
                left += 1;
                right -= 1;
            }
        }

        chars.iter().collect()
    }
}

// Test
fn main() {
    let mut s = "leetcode".to_string();
    println!("{}", Solution::reverse_vowels(s));

    s = "a.".to_string();
    println!("{}", Solution::reverse_vowels(s));
}
