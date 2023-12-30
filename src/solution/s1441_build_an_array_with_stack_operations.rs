/**
 * [1441] Build an Array With Stack Operations
 *
 * You are given an integer array target and an integer n.
 * You have an empty stack with the two following operations:
 *
 * 	"Push": pushes an integer to the top of the stack.
 * 	"Pop": removes the integer on the top of the stack.
 *
 * You also have a stream of the integers in the range [1, n].
 * Use the two stack operations to make the numbers in the stack (from the bottom to the top) equal to target. You should follow the following rules:
 *
 * 	If the stream of the integers is not empty, pick the next integer from the stream and push it to the top of the stack.
 * 	If the stack is not empty, pop the integer at the top of the stack.
 * 	If, at any moment, the elements in the stack (from the bottom to the top) are equal to target, do not read new integers from the stream and do not do more operations on the stack.
 *
 * Return the stack operations needed to build target following the mentioned rules. If there are multiple valid answers, return any of them.
 *  
 * Example 1:
 *
 * Input: target = [1,3], n = 3
 * Output: ["Push","Push","Pop","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Pop the integer on the top of the stack. s = [1].
 * Read 3 from the stream and push it to the stack. s = [1,3].
 *
 * Example 2:
 *
 * Input: target = [1,2,3], n = 3
 * Output: ["Push","Push","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Read 3 from the stream and push it to the stack. s = [1,2,3].
 *
 * Example 3:
 *
 * Input: target = [1,2], n = 4
 * Output: ["Push","Push"]
 * Explanation: Initially the stack s is empty. The last element is the top of the stack.
 * Read 1 from the stream and push it to the stack. s = [1].
 * Read 2 from the stream and push it to the stack. s = [1,2].
 * Since the stack (from the bottom to the top) is equal to target, we stop the stack operations.
 * The answers that read integer 3 from the stream are not accepted.
 *
 *  
 * Constraints:
 *
 * 	1 <= target.length <= 100
 * 	1 <= n <= 100
 * 	1 <= target[i] <= n
 * 	target is strictly increasing.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/build-an-array-with-stack-operations/
// discuss: https://leetcode.com/problems/build-an-array-with-stack-operations/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn build_array(target: Vec<i32>, n: i32) -> Vec<String> {
        let mut result = vec![];
        let mut index = 0;

        for i in 1..=n {
            let target_item = target[index];
            result.push("Push".to_string());

            if target_item != i {
                result.push("Pop".to_string());
                continue;
            }

            index += 1;

            if index == target.len() {
                break;
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
    fn test_1441_example_1() {
        let target = vec![1, 3];
        let n = 3;

        let result = vec_string!["Push", "Push", "Pop", "Push"];

        assert_eq!(Solution::build_array(target, n), result);
    }

    #[test]
    fn test_1441_example_2() {
        let target = vec![1, 2, 3];
        let n = 3;

        let result = vec_string!["Push", "Push", "Push"];

        assert_eq!(Solution::build_array(target, n), result);
    }

    #[test]
    fn test_1441_example_3() {
        let target = vec![1, 2];
        let n = 4;

        let result = vec_string!["Push", "Push"];

        assert_eq!(Solution::build_array(target, n), result);
    }
}
