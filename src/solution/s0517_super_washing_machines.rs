/**
 * [0517] Super Washing Machines
 *
 * You have n super washing machines on a line. Initially, each washing machine has some dresses or is empty.
 * For each move, you could choose any m (1 <= m <= n) washing machines, and pass one dress of each washing machine to one of its adjacent washing machines at the same time.
 * Given an integer array machines representing the number of dresses in each washing machine from left to right on the line, return the minimum number of moves to make all the washing machines have the same number of dresses. If it is not possible to do it, return -1.
 *  
 * Example 1:
 *
 * Input: machines = [1,0,5]
 * Output: 3
 * Explanation:
 * 1st move:    1     0 <-- 5    =>    1     1     4
 * 2nd move:    1 <-- 1 <-- 4    =>    2     1     3
 * 3rd move:    2     1 <-- 3    =>    2     2     2
 *
 * Example 2:
 *
 * Input: machines = [0,3,0]
 * Output: 2
 * Explanation:
 * 1st move:    0 <-- 3     0    =>    1     2     0
 * 2nd move:    1     2 --> 0    =>    1     1     1
 *
 * Example 3:
 *
 * Input: machines = [0,2,0]
 * Output: -1
 * Explanation:
 * It's impossible to make all three washing machines have the same number of dresses.
 *
 *  
 * Constraints:
 *
 * 	n == machines.length
 * 	1 <= n <= 10^4
 * 	0 <= machines[i] <= 10^5
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/super-washing-machines/
// discuss: https://leetcode.com/problems/super-washing-machines/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_min_moves(machines: Vec<i32>) -> i32 {
        if machines.len() == 0 {
            return -1;
        }

        let sum: i32 = machines.iter().sum();

        if sum % (machines.len() as i32) != 0 {
            return -1;
        }

        let target = sum / (machines.len() as i32);

        let mut to_right = 0;
        let mut result = 0;

        for machine in machines.iter() {
            to_right = machine + to_right - target;
            result = result.max((machine - target).max(to_right.abs()));
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0517_example_1() {
        let machines = vec![1, 0, 5];
        let result = 3;

        assert_eq!(Solution::find_min_moves(machines), result);
    }

    #[test]
    fn test_0517_example_2() {
        let machines = vec![0, 3, 0];
        let result = 2;

        assert_eq!(Solution::find_min_moves(machines), result);
    }

    #[test]
    fn test_0517_example_3() {
        let machines = vec![0, 2, 0];
        let result = -1;

        assert_eq!(Solution::find_min_moves(machines), result);
    }
}
