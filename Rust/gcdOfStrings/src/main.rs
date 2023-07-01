// Added
struct Solution {}

// Solution
impl Solution {
    pub fn gcd_of_strings(str1: String, str2: String) -> String {
        // Is div-able
        if str1.clone() + &str2 != str2.clone() + &str1 {
            return String::new(); // Returning an empty string
        }

        // Let's make str2 the shortest string
        let (mut str1, mut str2) = if str1.len() < str2.len() {
            (str2, str1)
        } else {
            (str1, str2)
        };

        // For as long as str1 starts with str2, and
        // str2 is not empty
        while str1.starts_with(&str2) && !str2.is_empty() {
            // Slicing str1, starting from the length of str2.
            str1 = str1[str2.len()..].to_string();
            // if str1 gets shorter than str2, swap
            if str1.len() < str2.len() {
                std::mem::swap(&mut str1, &mut str2);
            }
        }

        str1
    }
}

fn main() {
    let mut a = "ABCABC".to_string();
    let mut b = "ABC".to_string();
    println!("{}", Solution::gcd_of_strings(a, b)); // ABC

    a = "ABABAB".to_string();
    b = "ABAB".to_string();
    println!("{}", Solution::gcd_of_strings(a, b)); // AB

    a = "LEET".to_string();
    b = "CODE".to_string();
    println!("{}", Solution::gcd_of_strings(a, b)); //

    a = "ABCDEF".to_string();
    b = "ABC".to_string();
    println!("{}", Solution::gcd_of_strings(a, b)); //

    a = "AABBAABBAA".to_string();
    b = "AABB".to_string();
    println!("{}", Solution::gcd_of_strings(a, b)); //
}
