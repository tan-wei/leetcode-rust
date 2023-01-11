/**
 * [0941] Valid Mountain Array
 *
 * Given an array of integers arr, return true if and only if it is a valid mountain array.
 * Recall that arr is a mountain array if and only if:
 *
 * 	arr.length >= 3
 * 	There exists some i with 0 < i < arr.length - 1 such that:
 *
 * 		arr[0] < arr[1] < ... < arr[i - 1] < arr[i]
 * 		arr[i] > arr[i + 1] > ... > arr[arr.length - 1]
 *
 *
 * <img src="https://assets.leetcode.com/uploads/2019/10/20/hint_valid_mountain_array.png" width="500" />
 *  
 * Example 1:
 * Input: arr = [2,1]
 * Output: false
 * Example 2:
 * Input: arr = [3,5,5]
 * Output: false
 * Example 3:
 * Input: arr = [0,3,2,1]
 * Output: true
 *  
 * Constraints:
 *
 * 	1 <= arr.length <= 10^4
 * 	0 <= arr[i] <= 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/valid-mountain-array/
// discuss: https://leetcode.com/problems/valid-mountain-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

#[derive(PartialEq)]
enum MountainState {
    Initial,
    Increasing,
    Decreasing,
}

impl Solution {
    pub fn valid_mountain_array(arr: Vec<i32>) -> bool {
        let mut state = MountainState::Initial;
        let mut cur = arr[0];
        for next in arr.into_iter().skip(1) {
            state = match (state, next.cmp(&cur)) {
                (MountainState::Initial, std::cmp::Ordering::Greater) => MountainState::Increasing,
                (MountainState::Initial, std::cmp::Ordering::Less) => return false,
                (MountainState::Increasing, std::cmp::Ordering::Greater) => {
                    MountainState::Increasing
                }
                (MountainState::Decreasing, std::cmp::Ordering::Greater) => return false,
                (_, std::cmp::Ordering::Less) => MountainState::Decreasing,
                (_, std::cmp::Ordering::Equal) => return false,
            };
            cur = next;
        }
        state == MountainState::Decreasing
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0941_example_1() {
        let arr = vec![2, 1];
        let result = false;

        assert_eq!(Solution::valid_mountain_array(arr), result);
    }

    #[test]
    fn test_0941_example_2() {
        let arr = vec![3, 5, 5];
        let result = false;

        assert_eq!(Solution::valid_mountain_array(arr), result);
    }

    #[test]
    fn test_0941_example_3() {
        let arr = vec![0, 3, 2, 1];
        let result = true;

        assert_eq!(Solution::valid_mountain_array(arr), result);
    }

    #[test]
    fn test_0941_additional_1() {
        let arr = vec![4, 4, 3, 2, 1];
        let result = false;

        assert_eq!(Solution::valid_mountain_array(arr), result);
    }
}
