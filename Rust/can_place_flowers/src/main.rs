// Added
struct Solution {}

// Solution
impl Solution {
    pub fn can_place_flowers(flowerbed: Vec<i32>, n: i32) -> bool {
        let mut count = 0;
        let mut skip = 0;

        let len = flowerbed.len();

        for i in 0..len {
            if flowerbed[i] == 1 {
                skip = 1;
            } else {
                if skip == 0 && (i == len - 1 || flowerbed[i + 1] == 0) {
                    count += 1;
                    skip = 1;

                    if count >= n {
                        return true;
                    }
                } else {
                    skip = 0
                }
            }
        }

        count >= n
    }
}

// Test
fn main() {
    let mut flowerbed = [1, 0, 0, 0, 1].to_vec();
    let mut n = 1;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // true

    flowerbed = [1, 0, 0, 0, 1].to_vec();
    n = 2;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // false

    flowerbed = [1, 0, 0, 0, 0, 1].to_vec();
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // false

    flowerbed = [0, 0, 1, 0, 1].to_vec();
    n = 1;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // true

    flowerbed = [1, 0, 0, 0, 1, 0, 0].to_vec();
    n = 2;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // true

    flowerbed = [0, 0, 0, 0, 1, 0, 0].to_vec();
    n = 3;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // true

    flowerbed = [0].to_vec();
    n = 1;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // true

    flowerbed = [0, 1, 0].to_vec();
    n = 1;
    println!("{}", Solution::can_place_flowers(flowerbed, n)); // false
}
