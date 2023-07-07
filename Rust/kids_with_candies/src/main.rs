// Added
struct Solution {}

// Solution
impl Solution {
    pub fn kids_with_candies(candies: Vec<i32>, extra_candies: i32) -> Vec<bool> {
        // Iterate all elements and return the max
        // .unwrap returns an &i32, we use the '*' to dereference it
        let greatest: i32 = *candies.iter().max().unwrap();
        // 1. iterate through candies
        // 2. takes each element and transform it into the return value: true|false
        // 3. transform the result into a collection
        candies
            .iter()
            .map(|&c| c + extra_candies >= greatest)
            .collect()
    }
}

// Test
fn main() {
    let mut candies: Vec<i32> = [2, 3, 5, 1, 3].to_vec(); // Square brackets form an array, to create a vector add a ".to_vec()"
    let mut extra_candies: i32 = 3;
    println!("{:?}", Solution::kids_with_candies(candies, extra_candies)); // [true,true,true,false,true]

    candies = [4, 2, 1, 1, 2].to_vec();
    extra_candies = 1;
    println!("{:?}", Solution::kids_with_candies(candies, extra_candies)); // [true,false,false,false,false]

    candies = [12, 1, 12].to_vec();
    extra_candies = 10;
    println!("{:?}", Solution::kids_with_candies(candies, extra_candies)); // [true,false,true]
}
