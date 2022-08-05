/**
 * [0781] Rabbits in Forest
 *
 * There is a forest with an unknown number of rabbits. We asked n rabbits "How many rabbits have the same color as you?" and collected the answers in an integer array answers where answers[i] is the answer of the i^th rabbit.
 * Given the array answers, return the minimum number of rabbits that could be in the forest.
 *  
 * Example 1:
 *
 * Input: answers = [1,1,2]
 * Output: 5
 * Explanation:
 * The two rabbits that answered "1" could both be the same color, say red.
 * The rabbit that answered "2" can't be red or the answers would be inconsistent.
 * Say the rabbit that answered "2" was blue.
 * Then there should be 2 other blue rabbits in the forest that didn't answer into the array.
 * The smallest possible number of rabbits in the forest is therefore 5: 3 that answered plus 2 that didn't.
 *
 * Example 2:
 *
 * Input: answers = [10,10,10]
 * Output: 11
 *
 *  
 * Constraints:
 *
 * 	1 <= answers.length <= 1000
 * 	0 <= answers[i] < 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/rabbits-in-forest/
// discuss: https://leetcode.com/problems/rabbits-in-forest/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn num_rabbits(answers: Vec<i32>) -> i32 {
        answers
            .iter()
            .fold(std::collections::HashMap::new(), |mut acc, &x| {
                *acc.entry(x).or_insert(0) += 1;
                acc
            })
            .iter()
            .fold(0, |acc, (&key, &value)| {
                let teams = if value % (key + 1) == 0 {
                    value / (key + 1)
                } else {
                    value / (key + 1) + 1
                };

                acc + teams * (key + 1)
            })
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0781_example_1() {
        let answers = vec![1, 1, 2];
        let result = 5;

        assert_eq!(Solution::num_rabbits(answers), result)
    }

    #[test]
    fn test_0781_example_2() {
        let answers = vec![10, 10, 10];
        let result = 11;

        assert_eq!(Solution::num_rabbits(answers), result)
    }
}
