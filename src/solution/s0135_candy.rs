/**
 * [135] Candy
 *
 * There are n children standing in a line. Each child is assigned a rating value given in the integer array ratings.
 * You are giving candies to these children subjected to the following requirements:
 *
 * 	Each child must have at least one candy.
 * 	Children with a higher rating get more candies than their neighbors.
 *
 * Return the minimum number of candies you need to have to distribute the candies to the children.
 *  
 * Example 1:
 *
 * Input: ratings = [1,0,2]
 * Output: 5
 * Explanation: You can allocate to the first, second and third child with 2, 1, 2 candies respectively.
 *
 * Example 2:
 *
 * Input: ratings = [1,2,2]
 * Output: 4
 * Explanation: You can allocate to the first, second and third child with 1, 2, 1 candies respectively.
 * The third child gets 1 candy because it satisfies the above two conditions.
 *
 *  
 * Constraints:
 *
 * 	n == ratings.length
 * 	1 <= n <= 2 * 10^4
 * 	0 <= ratings[i] <= 2 * 10^4
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/candy/
// discuss: https://leetcode.com/problems/candy/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn candy(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();
        let mut allocation = vec![1; n];
        for i in 1..n {
            if ratings[i] > ratings[i - 1] {
                allocation[i] = allocation[i - 1] + 1;
            }
        }
        for i in (0..n - 1).rev() {
            if ratings[i] > ratings[i + 1] {
                allocation[i] = allocation[i].max(allocation[i + 1] + 1);
            }
        }
        allocation.iter().sum()
    }

    pub fn candy_v2(ratings: Vec<i32>) -> i32 {
        let n = ratings.len();

        if n <= 1 {
            return n as i32;
        }

        let mut result = 1;
        let mut pre = 1;
        let mut cnt = 0;

        for i in 1..n {
            if ratings[i] >= ratings[i - 1] {
                if cnt > 0 {
                    result += cnt * (cnt + 1) / 2;
                    if cnt >= pre {
                        result += cnt - pre + 1;
                    }
                    cnt = 0;
                    pre = 1;
                }
                pre = if ratings[i] == ratings[i - 1] {
                    1
                } else {
                    pre + 1
                };
                result += pre;
            } else {
                cnt += 1;
            }
        }

        if cnt > 0 {
            result += cnt * (cnt + 1) / 2;
            if cnt >= pre {
                result += cnt - pre + 1
            };
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0135_example_1() {
        let ratings = vec![1, 0, 2];
        let result = 5;

        assert_eq!(Solution::candy(ratings), result);

        let ratings = vec![1, 0, 2];
        let result = 5;

        assert_eq!(Solution::candy_v2(ratings), result);
    }

    #[test]
    fn test_0135_example_2() {
        let ratings = vec![1, 2, 2];
        let result = 4;

        assert_eq!(Solution::candy(ratings), result);

        let ratings = vec![1, 2, 2];
        let result = 4;

        assert_eq!(Solution::candy_v2(ratings), result);
    }
}
