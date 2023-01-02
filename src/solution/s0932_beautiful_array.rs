/**
 * [0932] Beautiful Array
 *
 * An array nums of length n is beautiful if:
 *
 * 	nums is a permutation of the integers in the range [1, n].
 * 	For every 0 <= i < j < n, there is no index k with i < k < j where 2 * nums[k] == nums[i] + nums[j].
 *
 * Given the integer n, return any beautiful array nums of length n. There will be at least one valid answer for the given n.
 *  
 * Example 1:
 * Input: n = 4
 * Output: [2,1,4,3]
 * Example 2:
 * Input: n = 5
 * Output: [3,1,2,5,4]
 *  
 * Constraints:
 *
 * 	1 <= n <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/beautiful-array/
// discuss: https://leetcode.com/problems/beautiful-array/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn beautiful_array(n: i32) -> Vec<i32> {
        let mut result = Vec::with_capacity(n as usize);
        result.push(1);
        while result.len() < n as usize {
            result = result
                .iter()
                .map(|m| m * 2 - 1)
                .chain(result.iter().map(|m| m * 2))
                .filter(|&m| m <= n)
                .collect();
        }
        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_0932_example_1() {
        let n = 4;
        let result = vec![2, 1, 4, 3];

        assert_eq!(Solution::beautiful_array(n), result);
    }

    #[test]
    #[ignore]
    fn test_0932_example_2() {
        let n = 5;
        let result = vec![3, 1, 2, 5, 4];

        assert_eq!(Solution::beautiful_array(n), result);
    }
}
