/**
 * [1207] Unique Number of Occurrences
 *
 * Given an array of integers arr, return true if the number of occurrences of each value in the array is unique or false otherwise.
 *  
 * Example 1:
 *
 * Input: arr = [1,2,2,1,1,3]
 * Output: true
 * Explanation: The value 1 has 3 occurrences, 2 has 2 and 3 has 1. No two values have the same number of occurrences.
 * Example 2:
 *
 * Input: arr = [1,2]
 * Output: false
 *
 * Example 3:
 *
 * Input: arr = [-3,0,1,-3,1,1,1,-3,10,0]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 1000
 * 	-1000 <= arr[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/unique-number-of-occurrences/
// discuss: https://leetcode.com/problems/unique-number-of-occurrences/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn unique_occurrences(arr: Vec<i32>) -> bool {
        let (mut counter, mut set) = ([0; 2001], [false; 1001]);
        arr.iter().for_each(|&x| counter[(x + 1000) as usize] += 1);
        for &x in counter.iter().filter(|&&x| x > 0) {
            match set[x as usize] {
                true => return false,
                false => set[x as usize] = true,
            }
        }
        true
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1207_example_1() {
        let arr = vec![1, 2, 2, 1, 1, 3];
        let result = true;

        assert_eq!(Solution::unique_occurrences(arr), result);
    }

    #[test]
    fn test_1207_example_2() {
        let arr = vec![1, 2];
        let result = false;

        assert_eq!(Solution::unique_occurrences(arr), result);
    }

    #[test]
    fn test_1207_example_3() {
        let arr = vec![-3, 0, 1, -3, 1, 1, 1, -3, 10, 0];
        let result = true;

        assert_eq!(Solution::unique_occurrences(arr), result);
    }
}
