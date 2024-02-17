/**
 * [1503] Last Moment Before All Ants Fall Out of a Plank
 *
 * We have a wooden plank of the length n units. Some ants are walking on the plank, each ant moves with a speed of 1 unit per second. Some of the ants move to the left, the other move to the right.
 * When two ants moving in two different directions meet at some point, they change their directions and continue moving again. Assume changing directions does not take any additional time.
 * When an ant reaches one end of the plank at a time t, it falls out of the plank immediately.
 * Given an integer n and two integer arrays left and right, the positions of the ants moving to the left and the right, return the moment when the last ant(s) fall out of the plank.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/17/ants.jpg" style="width: 450px; height: 610px;" />
 * Input: n = 4, left = [4,3], right = [0,1]
 * Output: 4
 * Explanation: In the image above:
 * -The ant at index 0 is named A and going to the right.
 * -The ant at index 1 is named B and going to the right.
 * -The ant at index 3 is named C and going to the left.
 * -The ant at index 4 is named D and going to the left.
 * The last moment when an ant was on the plank is t = 4 seconds. After that, it falls immediately out of the plank. (i.e., We can say that at t = 4.0000000001, there are no ants on the plank).
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/17/ants2.jpg" style="width: 639px; height: 101px;" />
 * Input: n = 7, left = [], right = [0,1,2,3,4,5,6,7]
 * Output: 7
 * Explanation: All ants are going to the right, the ant at index 0 needs 7 seconds to fall.
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/06/17/ants3.jpg" style="width: 639px; height: 100px;" />
 * Input: n = 7, left = [0,1,2,3,4,5,6,7], right = []
 * Output: 7
 * Explanation: All ants are going to the left, the ant at index 7 needs 7 seconds to fall.
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^4
 * 	0 <= left.length <= n + 1
 * 	0 <= left[i] <= n
 * 	0 <= right.length <= n + 1
 * 	0 <= right[i] <= n
 * 	1 <= left.length + right.length <= n + 1
 * 	All values of left and right are unique, and each value can appear only in one of the two arrays.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/
// discuss: https://leetcode.com/problems/last-moment-before-all-ants-fall-out-of-a-plank/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn get_last_moment(n: i32, left: Vec<i32>, right: Vec<i32>) -> i32 {
        left.into_iter()
            .max()
            .unwrap_or(0)
            .max(n - right.into_iter().min().unwrap_or(n))
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1503_example_1() {
        let n = 4;
        let left = vec![4, 3];
        let right = vec![0, 1];

        let result = 4;

        assert_eq!(Solution::get_last_moment(n, left, right), result);
    }

    #[test]
    fn test_1503_example_2() {
        let n = 7;
        let left = vec![];
        let right = vec![0, 1, 2, 3, 4, 5, 6, 7];

        let result = 7;

        assert_eq!(Solution::get_last_moment(n, left, right), result);
    }

    #[test]
    fn test_1503_example_3() {
        let n = 7;
        let left = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let right = vec![];

        let result = 7;

        assert_eq!(Solution::get_last_moment(n, left, right), result);
    }
}
