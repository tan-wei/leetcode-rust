/**
 * [2279] Maximum Bags With Full Capacity of Rocks
 *
 * You have n bags numbered from 0 to n - 1. You are given two 0-indexed integer arrays capacity and rocks. The i^th bag can hold a maximum of capacity[i] rocks and currently contains rocks[i] rocks. You are also given an integer additionalRocks, the number of additional rocks you can place in any of the bags.
 * Return the maximum number of bags that could have full capacity after placing the additional rocks in some bags.
 *  
 * Example 1:
 *
 * Input: capacity = [2,3,4,5], rocks = [1,2,4,4], additionalRocks = 2
 * Output: 3
 * Explanation:
 * Place 1 rock in bag 0 and 1 rock in bag 1.
 * The number of rocks in each bag are now [2,3,4,4].
 * Bags 0, 1, and 2 have full capacity.
 * There are 3 bags at full capacity, so we return 3.
 * It can be shown that it is not possible to have more than 3 bags at full capacity.
 * Note that there may be other ways of placing the rocks that result in an answer of 3.
 *
 * Example 2:
 *
 * Input: capacity = [10,2,2], rocks = [2,2,0], additionalRocks = 100
 * Output: 3
 * Explanation:
 * Place 8 rocks in bag 0 and 2 rocks in bag 2.
 * The number of rocks in each bag are now [10,2,2].
 * Bags 0, 1, and 2 have full capacity.
 * There are 3 bags at full capacity, so we return 3.
 * It can be shown that it is not possible to have more than 3 bags at full capacity.
 * Note that we did not use all of the additional rocks.
 *
 *  
 * Constraints:
 *
 * 	n == capacity.length == rocks.length
 * 	1 <= n <= 5 * 10^4
 * 	1 <= capacity[i] <= 10^9
 * 	0 <= rocks[i] <= capacity[i]
 * 	1 <= additionalRocks <= 10^9
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/
// discuss: https://leetcode.com/problems/maximum-bags-with-full-capacity-of-rocks/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn maximum_bags(capacity: Vec<i32>, rocks: Vec<i32>, additional_rocks: i32) -> i32 {
        0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_2279_example_1() {
        let capacity = vec![2, 3, 4, 5];
        let rocks = vec![1, 2, 4, 4];
        let additional_rocks = 2;

        let result = 3;

        assert_eq!(
            Solution::maximum_bags(capacity, rocks, additional_rocks),
            result
        );
    }

    #[test]
    #[ignore]
    fn test_2279_example_2() {
        let capacity = vec![10, 2, 2];
        let rocks = vec![2, 2, 0];
        let additional_rocks = 100;

        let result = 3;

        assert_eq!(
            Solution::maximum_bags(capacity, rocks, additional_rocks),
            result
        );
    }
}
