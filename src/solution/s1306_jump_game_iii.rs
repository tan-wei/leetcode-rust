/**
 * [1306] Jump Game III
 *
 * Given an array of non-negative integers arr, you are initially positioned at start index of the array. When you are at index i, you can jump to i + arr[i] or i - arr[i], check if you can reach any index with value 0.
 * Notice that you can not jump outside of the array at any time.
 *
 * Example 1:
 *
 * Input: arr = [4,2,3,0,3,1,2], start = 5
 * Output: true
 * Explanation:
 * All possible ways to reach at index 3 with value 0 are:
 * index 5 -> index 4 -> index 1 -> index 3
 * index 5 -> index 6 -> index 4 -> index 1 -> index 3
 *
 * Example 2:
 *
 * Input: arr = [4,2,3,0,3,1,2], start = 0
 * Output: true
 * Explanation:
 * One possible way to reach at index 3 with value 0 is:
 * index 0 -> index 4 -> index 1 -> index 3
 *
 * Example 3:
 *
 * Input: arr = [3,0,2,1,2], start = 2
 * Output: false
 * Explanation: There is no way to reach at index 1 with value 0.
 *
 *
 * Constraints:
 *
 * 	1 <= arr.length <= 5 * 10^4
 * 	0 <= arr[i] < arr.length
 * 	0 <= start < arr.length
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/jump-game-iii/
// discuss: https://leetcode.com/problems/jump-game-iii/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/jump-game-iii/solutions/953386/rust-solution/
    pub fn can_reach(arr: Vec<i32>, start: i32) -> bool {
        let mut arr = arr;
        let mut vd = std::collections::VecDeque::with_capacity(arr.len());

        vd.push_back(start);

        while let Some(front) = vd.pop_front() {
            match arr[front as usize].cmp(&0) {
                std::cmp::Ordering::Less => continue,
                std::cmp::Ordering::Equal => return true,
                std::cmp::Ordering::Greater => {
                    if front - arr[front as usize] >= 0 {
                        vd.push_back(front - arr[front as usize]);
                    }
                    if front + arr[front as usize] < arr.len() as i32 {
                        vd.push_back(front + arr[front as usize]);
                    }
                    arr[front as usize] = -1;
                }
            }
        }

        false
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1306_example_1() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 5;
        let result = true;

        assert_eq!(Solution::can_reach(arr, start), result);
    }

    #[test]
    fn test_1306_example_2() {
        let arr = vec![4, 2, 3, 0, 3, 1, 2];
        let start = 0;
        let result = true;

        assert_eq!(Solution::can_reach(arr, start), result);
    }

    #[test]
    fn test_1306_example_3() {
        let arr = vec![3, 0, 2, 1, 2];
        let start = 2;
        let result = false;

        assert_eq!(Solution::can_reach(arr, start), result);
    }
}
