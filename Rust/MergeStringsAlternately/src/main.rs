// Added
struct Solution {}

// Solution
impl Solution {
    pub fn merge_alternately(word1: String, word2: String) -> String {
        let mut res = String::new();
        let mut w1 = word1;
        let mut w2 = word2;

        while !w1.is_empty() && !w2.is_empty() {
            res.push(w1.remove(0));
            res.push(w2.remove(0));
        }

        res.push_str(&w1);
        res.push_str(&w2);

        res
    }
}

// Test
fn main() {
    let a = "ace".to_string();
    let b = "bdfgh".to_string();

    println!("{}", Solution::merge_alternately(a, b));
}
