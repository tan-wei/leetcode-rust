/**
 * [0553] Optimal Division
 *
 * You are given an integer array nums. The adjacent integers in nums will perform the float division.
 *
 * 	For example, for nums = [2,3,4], we will evaluate the expression "2/3/4".
 *
 * However, you can add any number of parenthesis at any position to change the priority of operations. You want to add these parentheses such the value of the expression after the evaluation is maximum.
 * Return the corresponding expression that has the maximum value in string format.
 * Note: your expression should not contain redundant parenthesis.
 *  
 * Example 1:
 *
 * Input: nums = [1000,100,10,2]
 * Output: "1000/(100/10/2)"
 * Explanation:
 * 1000/(100/10/2) = 1000/((100/10)/2) = 200
 * However, the bold parenthesis in "1000/((100/10)/2)" are redundant, since they don't influence the operation priority. So you should return "1000/(100/10/2)".
 * Other cases:
 * 1000/(100/10)/2 = 50
 * 1000/(100/(10/2)) = 50
 * 1000/100/10/2 = 0.5
 * 1000/100/(10/2) = 2
 *
 * Example 2:
 *
 * Input: nums = [2,3,4]
 * Output: "2/(3/4)"
 *
 * Example 3:
 *
 * Input: nums = [2]
 * Output: "2"
 *
 *  
 * Constraints:
 *
 * 	1 <= nums.length <= 10
 * 	2 <= nums[i] <= 1000
 * 	There is only one optimal division for the given iput.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/optimal-division/
// discuss: https://leetcode.com/problems/optimal-division/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn optimal_division(nums: Vec<i32>) -> String {
        let mut result = String::new();
        for (index, &num) in nums.iter().enumerate() {
            if index > 0 {
                result.push('/');
            }

            if nums.len() > 2 && index == 1 {
                result.push('(');
            }

            result.push_str(&num.to_string());

            if nums.len() > 2 && index + 1 == nums.len() {
                result.push(')');
            }
        }

        result
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0553_example_1() {
        let nums = vec![1000, 100, 10, 2];
        let result = "1000/(100/10/2)".to_string();

        assert_eq!(Solution::optimal_division(nums), result);
    }

    #[test]
    fn test_0553_example_2() {
        let nums = vec![2, 3, 4];
        let result = "2/(3/4)".to_string();

        assert_eq!(Solution::optimal_division(nums), result);
    }

    #[test]
    fn test_0553_example_3() {
        let nums = vec![2];
        let result = "2".to_string();

        assert_eq!(Solution::optimal_division(nums), result);
    }
}
