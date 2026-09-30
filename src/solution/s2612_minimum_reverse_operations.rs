/**
 * [2612] Minimum Reverse Operations
 *
 * You are given an integer n and an integer p representing an array arr of length n where all elements are set to 0's, except position p which is set to 1. You are also given an integer array banned containing restricted positions. Perform the following operation on arr:
 *
 * 	Reverse a subarray with size k if the single 1 is not set to a position in banned.
 *
 * Return an integer array answer with n results where the i^th result is the minimum number of operations needed to bring the single 1 to position i in arr, or -1 if it is impossible.
 *  
 * Example 1:
 *
 * Input: n = 4, p = 0, banned = [1,2], k = 4
 * Output: [0,-1,-1,1]
 * Explanation:
 *
 * 	Initially 1 is placed at position 0 so the number of operations we need for position 0 is 0.
 * 	We can never place 1 on the banned positions, so the answer for positions 1 and 2 is -1.
 * 	Perform the operation of size 4 to reverse the whole array.
 * 	After a single operation 1 is at position 3 so the answer for position 3 is 1.
 *
 * Example 2:
 *
 * Input: n = 5, p = 0, banned = [2,4], k = 3
 * Output: [0,-1,-1,-1,-1]
 * Explanation:
 *
 * 	Initially 1 is placed at position 0 so the number of operations we need for position 0 is 0.
 * 	We cannot perform the operation on the subarray positions [0, 2] because position 2 is in banned.
 * 	Because 1 cannot be set at position 2, it is impossible to set 1 at other positions in more operations.
 *
 * Example 3:
 *
 * Input: n = 4, p = 2, banned = [0,1,3], k = 1
 * Output: [-1,-1,0,-1]
 * Explanation:
 * Perform operations of size 1 and 1 never changes its position.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^5
 * 	0 <= p <= n - 1
 * 	0 <= banned.length <= n - 1
 * 	0 <= banned[i] <= n - 1
 * 	1 <= k <= n
 * 	banned[i] != p
 * 	all values in banned are unique
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-reverse-operations/
// discuss: https://leetcode.com/problems/minimum-reverse-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn min_reverse_operations(n: i32, p: i32, banned: Vec<i32>, k: i32) -> Vec<i32> {
        vec![]
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2612_example_1() {
        let n = 4;
        let p = 0;
        let banned = vec![1, 2];
        let k = 4;

        let result = vec![0, -1, -1, 1];

        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), result);
    }

    #[test]
    #[ignore]
    fn test_2612_example_2() {
        let n = 5;
        let p = 0;
        let banned = vec![2, 4];
        let k = 3;

        let result = vec![0, -1, -1, -1, -1];

        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), result);
    }

    #[test]
    #[ignore]
    fn test_2612_example_3() {
        let n = 4;
        let p = 2;
        let banned = vec![0, 1, 3];
        let k = 1;

        let result = vec![-1, -1, 0, -1];

        assert_eq!(Solution::min_reverse_operations(n, p, banned, k), result);
    }
}
