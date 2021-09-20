/**
 * [335] Self Crossing
 *
 * You are given an array of integers distance.
 * You start at point (0,0) on an X-Y plane and you move distance[0] meters to the north, then distance[1] meters to the west, distance[2] meters to the south, distance[3] meters to the east, and so on. In other words, after each move, your direction changes counter-clockwise.
 * Return true if your path crosses itself, and false if it does not.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross1-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [2,1,1,2]
 * Output: true
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross2-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [1,2,3,4]
 * Output: false
 *
 * Example 3:
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/03/14/selfcross3-plane.jpg" style="width: 400px; height: 435px;" />
 * Input: distance = [1,1,1,1]
 * Output: true
 *
 *  
 * Constraints:
 *
 * 	1 <= distance.length <= 10^5
 * 	1 <= distance[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/self-crossing/
// discuss: https://leetcode.com/problems/self-crossing/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/self-crossing/discuss/79131/Java-Oms-with-explanation
    pub fn is_self_crossing(distance: Vec<i32>) -> bool {
        let l = distance.len();
        if l <= 3 {
            return false;
        }

        for i in 3..l {
            if distance[i] >= distance[i - 2] && distance[i - 1] <= distance[i - 3] {
                //Fourth line crosses first line and onward
                return true;
            }
            if i >= 4 {
                if distance[i - 1] == distance[i - 3]
                    && distance[i] + distance[i - 4] >= distance[i - 2]
                {
                    // Fifth line meets first line and onward
                    return true;
                }
            }
            if i >= 5 {
                if distance[i - 2] - distance[i - 4] >= 0
                    && distance[i] >= distance[i - 2] - distance[i - 4]
                    && distance[i - 1] >= distance[i - 3] - distance[i - 5]
                    && distance[i - 1] <= distance[i - 3]
                {
                    // Sixth line crosses first line and onward
                    return true;
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
    fn test_0335_example_1() {
        let distance = vec![2, 1, 1, 2];
        let result = true;

        assert_eq!(Solution::is_self_crossing(distance), result);
    }

    #[test]
    fn test_0335_example_2() {
        let distance = vec![1, 2, 3, 4];
        let result = false;

        assert_eq!(Solution::is_self_crossing(distance), result);
    }

    #[test]
    fn test_0335_example_3() {
        let distance = vec![1, 1, 1, 1];
        let result = true;

        assert_eq!(Solution::is_self_crossing(distance), result);
    }
}
