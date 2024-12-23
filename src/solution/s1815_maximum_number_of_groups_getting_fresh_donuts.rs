/**
 * [1815] Maximum Number of Groups Getting Fresh Donuts
 *
 * There is a donuts shop that bakes donuts in batches of batchSize. They have a rule where they must serve all of the donuts of a batch before serving any donuts of the next batch. You are given an integer batchSize and an integer array groups, where groups[i] denotes that there is a group of groups[i] customers that will visit the shop. Each customer will get exactly one donut.
 * When a group visits the shop, all customers of the group must be served before serving any of the following groups. A group will be happy if they all get fresh donuts. That is, the first customer of the group does not receive a donut that was left over from the previous group.
 * You can freely rearrange the ordering of the groups. Return the maximum possible number of happy groups after rearranging the groups.
 *  
 * Example 1:
 *
 * Input: batchSize = 3, groups = [1,2,3,4,5,6]
 * Output: 4
 * Explanation: You can arrange the groups as [6,2,4,5,1,3]. Then the 1^st, 2^nd, 4^th, and 6^th groups will be happy.
 *
 * Example 2:
 *
 * Input: batchSize = 4, groups = [1,3,2,5,2,2,1,6]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= batchSize <= 9
 * 	1 <= groups.length <= 30
 * 	1 <= groups[i] <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-number-of-groups-getting-fresh-donuts/
// discuss: https://leetcode.com/problems/maximum-number-of-groups-getting-fresh-donuts/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_happy_groups(batch_size: i32, groups: Vec<i32>) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_1815_example_1() {
        let batch_size = 3;
        let groups = vec![1, 2, 3, 4, 5, 6];

        let result = 4;

        assert_eq!(Solution::max_happy_groups(batch_size, groups), result);
    }

    #[test]
    #[ignore]
    fn test_1815_example_2() {
        let batch_size = 4;
        let groups = vec![1, 3, 2, 5, 2, 2, 1, 6];

        let result = 4;

        assert_eq!(Solution::max_happy_groups(batch_size, groups), result);
    }
}
