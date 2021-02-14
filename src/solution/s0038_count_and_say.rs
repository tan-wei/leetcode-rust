/**
 * [38] Count and Say
 *
 * The count-and-say sequence is a sequence of digit strings defined by the recursive formula:
 *
 * 	countAndSay(1) = "1"
 * 	countAndSay(n) is the way you would "say" the digit string from countAndSay(n-1), which is then converted into a different digit string.
 *
 * To determine how you "say" a digit string, split it into the minimal number of groups so that each group is a contiguous section all of the same character. Then for each group, say the number of characters, then say the character. To convert the saying into a digit string, replace the counts with a number and concatenate every saying.
 * For example, the saying and conversion for digit string "3322251":
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/10/23/countandsay.jpg" style="width: 581px; height: 172px;" />
 * Given a positive integer n, return the n^th term of the count-and-say sequence.
 *  
 * Example 1:
 *
 * Input: n = 1
 * Output: "1"
 * Explanation: This is the base case.
 *
 * Example 2:
 *
 * Input: n = 4
 * Output: "1211"
 * Explanation:
 * countAndSay(1) = "1"
 * countAndSay(2) = say "1" = one 1 = "11"
 * countAndSay(3) = say "11" = two 1's = "21"
 * countAndSay(4) = say "21" = one 2 + one 1 = "12" + "11" = "1211"
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 30
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/count-and-say/
// discuss: https://leetcode.com/problems/count-and-say/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        if n == 1 {
            return "1".to_string();
        }

        let prev_res = Self::count_and_say(n - 1).chars().collect::<Vec<char>>();
        let mut res = vec![];

        let mut prev_chr = &prev_res[0];
        let mut count = 1;

        for chr in &prev_res[1..] {
            if chr == prev_chr {
                count += 1;
            } else {
                res.push(format!("{}{}", count, prev_chr));
                count = 1;
            }
            prev_chr = chr;
        }
        res.push(format!("{}{}", count, prev_chr));
        res.join("")
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0038_example_1() {
        assert_eq!(Solution::count_and_say(1), "1".to_string());
    }

    #[test]
    fn test_0038_example_2() {
        assert_eq!(Solution::count_and_say(4), "1211".to_string());
    }
}
