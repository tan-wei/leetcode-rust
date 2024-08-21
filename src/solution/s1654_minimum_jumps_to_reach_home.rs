/**
 * [1654] Minimum Jumps to Reach Home
 *
 * A certain bug's home is on the x-axis at position x. Help them get there from position 0.
 * The bug jumps according to the following rules:
 *
 * 	It can jump exactly a positions forward (to the right).
 * 	It can jump exactly b positions backward (to the left).
 * 	It cannot jump backward twice in a row.
 * 	It cannot jump to any forbidden positions.
 *
 * The bug may jump forward beyond its home, but it cannot jump to positions numbered with negative integers.
 * Given an array of integers forbidden, where forbidden[i] means that the bug cannot jump to the position forbidden[i], and integers a, b, and x, return the minimum number of jumps needed for the bug to reach its home. If there is no possible sequence of jumps that lands the bug on position x, return -1.
 *  
 * Example 1:
 *
 * Input: forbidden = [14,4,18,1,15], a = 3, b = 15, x = 9
 * Output: 3
 * Explanation: 3 jumps forward (0 -> 3 -> 6 -> 9) will get the bug home.
 *
 * Example 2:
 *
 * Input: forbidden = [8,3,16,6,12,20], a = 15, b = 13, x = 11
 * Output: -1
 *
 * Example 3:
 *
 * Input: forbidden = [1,6,2,14,5,17,4], a = 16, b = 9, x = 7
 * Output: 2
 * Explanation: One jump forward (0 -> 16) then one jump backward (16 -> 7) will get the bug home.
 *
 *  
 * Constraints:
 *
 * 	1 <= forbidden.length <= 1000
 * 	1 <= a, b, forbidden[i] <= 2000
 * 	0 <= x <= 2000
 * 	All the elements in forbidden are distinct.
 * 	Position x is not forbidden.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/minimum-jumps-to-reach-home/
// discuss: https://leetcode.com/problems/minimum-jumps-to-reach-home/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn minimum_jumps(forbidden: Vec<i32>, a: i32, b: i32, x: i32) -> i32 {
        if x == 0 {
            return 0;
        }
        let a = a as usize;
        let b = b as usize;
        let x = x as usize;

        let mut visited = vec![0; 6000];
        visited[0] = 2;
        for i in forbidden.iter() {
            visited[(*i) as usize] = 2;
        }
        let mut q = std::collections::VecDeque::new();

        q.push_back((0, 0, true));

        while let Some((p, c, forward)) = q.pop_front() {
            if p + a == x {
                return c + 1;
            }
            if p + a < 6000 && visited[p + a] != 2 {
                visited[p + a] = 2;
                q.push_back((p + a, c + 1, true));
            }

            if forward && p > b {
                if p - b == x {
                    return c + 1;
                }
                if visited[p - b] == 0 {
                    visited[p - b] = 1;
                    q.push_back((p - b, c + 1, false));
                }
            }
        }

        -1
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1654_example_1() {
        let forbidden = vec![14, 4, 18, 1, 15];
        let a = 3;
        let b = 15;
        let x = 9;

        let result = 3;

        assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), result);
    }

    #[test]
    fn test_1654_example_2() {
        let forbidden = vec![8, 3, 16, 6, 12, 20];
        let a = 15;
        let b = 13;
        let x = 11;

        let result = -1;

        assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), result);
    }

    #[test]
    fn test_1654_example_3() {
        let forbidden = vec![1, 6, 2, 14, 5, 17, 4];
        let a = 16;
        let b = 9;
        let x = 7;

        let result = 2;

        assert_eq!(Solution::minimum_jumps(forbidden, a, b, x), result);
    }
}
