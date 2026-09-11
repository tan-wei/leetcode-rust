/**
 * [2591] Distribute Money to Maximum Children
 *
 * You are given an integer money denoting the amount of money (in dollars) that you have and another integer children denoting the number of children that you must distribute the money to.
 * You have to distribute the money according to the following rules:
 *
 * 	All money must be distributed.
 * 	Everyone must receive at least 1 dollar.
 * 	Nobody receives 4 dollars.
 *
 * Return the maximum number of children who may receive exactly 8 dollars if you distribute the money according to the aforementioned rules. If there is no way to distribute the money, return -1.
 *  
 * Example 1:
 *
 * Input: money = 20, children = 3
 * Output: 1
 * Explanation:
 * The maximum number of children with 8 dollars will be 1. One of the ways to distribute the money is:
 * - 8 dollars to the first child.
 * - 9 dollars to the second child.
 * - 3 dollars to the third child.
 * It can be proven that no distribution exists such that number of children getting 8 dollars is greater than 1.
 *
 * Example 2:
 *
 * Input: money = 16, children = 2
 * Output: 2
 * Explanation: Each child can be given 8 dollars.
 *
 *  
 * Constraints:
 *
 * 	1 <= money <= 200
 * 	2 <= children <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/distribute-money-to-maximum-children/
// discuss: https://leetcode.com/problems/distribute-money-to-maximum-children/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn dist_money(money: i32, children: i32) -> i32 {
        if money < children {
            return -1;
        }

        let drawback = (children * 8 - money) as f32;

        if drawback == 0_f32 {
            return children;
        }

        if drawback == 4_f32 {
            return children - 2;
        }

        if drawback < 0_f32 {
            return children - 1;
        }

        children - (drawback / 7_f32).ceil() as i32
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2591_example_1() {
        let money = 20;
        let children = 3;

        let result = 1;

        assert_eq!(Solution::dist_money(money, children), result);
    }

    #[test]
    fn test_2591_example_2() {
        let money = 16;
        let children = 2;

        let result = 2;

        assert_eq!(Solution::dist_money(money, children), result);
    }
}
