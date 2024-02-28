/**
 * [1518] Water Bottles
 *
 * There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.
 * The operation of drinking a full water bottle turns it into an empty bottle.
 * Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/01/sample_1_1875.png" style="width: 500px; height: 245px;" />
 * Input: numBottles = 9, numExchange = 3
 * Output: 13
 * Explanation: You can exchange 3 empty bottles to get 1 full water bottle.
 * Number of water bottles you can drink: 9 + 3 + 1 = 13.
 *
 * Example 2:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/07/01/sample_2_1875.png" style="width: 500px; height: 183px;" />
 * Input: numBottles = 15, numExchange = 4
 * Output: 19
 * Explanation: You can exchange 4 empty bottles to get 1 full water bottle.
 * Number of water bottles you can drink: 15 + 3 + 1 = 19.
 *
 *  
 * Constraints:
 *
 * 	1 <= numBottles <= 100
 * 	2 <= numExchange <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/water-bottles/
// discuss: https://leetcode.com/problems/water-bottles/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_water_bottles(num_bottles: i32, num_exchange: i32) -> i32 {
        let mut drank = 0;
        let mut num_bottles = num_bottles;

        while num_bottles >= num_exchange {
            let rem = num_bottles % num_exchange;
            drank += num_bottles - rem;
            num_bottles = (num_bottles / num_exchange) + rem;
        }
        drank + num_bottles
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1518_example_1() {
        let num_bottles = 9;
        let num_exchange = 3;

        let result = 13;

        assert_eq!(
            Solution::num_water_bottles(num_bottles, num_exchange),
            result
        );
    }

    #[test]
    fn test_1518_example_2() {
        let num_bottles = 15;
        let num_exchange = 4;

        let result = 19;

        assert_eq!(
            Solution::num_water_bottles(num_bottles, num_exchange),
            result
        );
    }
}
