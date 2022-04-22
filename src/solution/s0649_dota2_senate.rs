/**
 * [0649] Dota2 Senate
 *
 * In the world of Dota2, there are two parties: the Radiant and the Dire.
 * The Dota2 senate consists of senators coming from two parties. Now the Senate wants to decide on a change in the Dota2 game. The voting for this change is a round-based procedure. In each round, each senator can exercise one of the two rights:
 *
 * 	Ban one senator's right: A senator can make another senator lose all his rights in this and all the following rounds.
 * 	Announce the victory: If this senator found the senators who still have rights to vote are all from the same party, he can announce the victory and decide on the change in the game.
 *
 * Given a string senate representing each senator's party belonging. The character 'R' and 'D' represent the Radiant party and the Dire party. Then if there are n senators, the size of the given string will be n.
 * The round-based procedure starts from the first senator to the last senator in the given order. This procedure will last until the end of voting. All the senators who have lost their rights will be skipped during the procedure.
 * Suppose every senator is smart enough and will play the best strategy for his own party. Predict which party will finally announce the victory and change the Dota2 game. The output should be "Radiant" or "Dire".
 *  
 * Example 1:
 *
 * Input: senate = "RD"
 * Output: "Radiant"
 * Explanation:
 * The first senator comes from Radiant and he can just ban the next senator's right in round 1.
 * And the second senator can't exercise any rights anymore since his right has been banned.
 * And in round 2, the first senator can just announce the victory since he is the only guy in the senate who can vote.
 *
 * Example 2:
 *
 * Input: senate = "RDD"
 * Output: "Dire"
 * Explanation:
 * The first senator comes from Radiant and he can just ban the next senator's right in round 1.
 * And the second senator can't exercise any rights anymore since his right has been banned.
 * And the third senator comes from Dire and he can ban the first senator's right in round 1.
 * And in round 2, the third senator can just announce the victory since he is the only guy in the senate who can vote.
 *
 *  
 * Constraints:
 *
 * 	n == senate.length
 * 	1 <= n <= 10^4
 * 	senate[i] is either 'R' or 'D'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/dota2-senate/
// discuss: https://leetcode.com/problems/dota2-senate/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/dota2-senate/discuss/912324/Rust-0-MS
    pub fn predict_party_victory(senate: String) -> String {
        let mut queue1 = std::collections::VecDeque::new();
        let mut queue2 = std::collections::VecDeque::new();

        for (index, item) in senate.chars().enumerate() {
            if item == 'R' {
                queue1.push_back(index);
            } else {
                queue2.push_back(index);
            }
        }

        while !queue1.is_empty() && !queue2.is_empty() {
            if let Some(r_index) = queue1.pop_front() {
                if let Some(d_index) = queue2.pop_front() {
                    if r_index < d_index {
                        queue1.push_back(r_index + senate.len());
                    } else {
                        queue2.push_back(d_index + senate.len());
                    }
                }
            }
        }
        if queue1.len() > queue2.len() {
            String::from("Radiant")
        } else {
            String::from("Dire")
        }
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0649_example_1() {
        let senate = "RD".to_string();
        let result = "Radiant".to_string();

        assert_eq!(Solution::predict_party_victory(senate), result);
    }

    #[test]
    fn test_0649_example_2() {
        let senate = "RDD".to_string();
        let result = "Dire".to_string();

        assert_eq!(Solution::predict_party_victory(senate), result);
    }
}
