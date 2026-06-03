/**
 * [2491] Divide Players Into Teams of Equal Skill
 *
 * You are given a positive integer array skill of even length n where skill[i] denotes the skill of the i^th player. Divide the players into n / 2 teams of size 2 such that the total skill of each team is equal.
 * The chemistry of a team is equal to the product of the skills of the players on that team.
 * Return the sum of the chemistry of all the teams, or return -1 if there is no way to divide the players into teams such that the total skill of each team is equal.
 *  
 * Example 1:
 *
 * Input: skill = [3,2,5,1,3,4]
 * Output: 22
 * Explanation:
 * Divide the players into the following teams: (1, 5), (2, 4), (3, 3), where each team has a total skill of 6.
 * The sum of the chemistry of all the teams is: 1 * 5 + 2 * 4 + 3 * 3 = 5 + 8 + 9 = 22.
 *
 * Example 2:
 *
 * Input: skill = [3,4]
 * Output: 12
 * Explanation:
 * The two players form a team with a total skill of 7.
 * The chemistry of the team is 3 * 4 = 12.
 *
 * Example 3:
 *
 * Input: skill = [1,1,2,3]
 * Output: -1
 * Explanation:
 * There is no way to divide the players into teams such that the total skill of each team is equal.
 *
 *  
 * Constraints:
 *
 * 	2 <= skill.length <= 10^5
 * 	skill.length is even.
 * 	1 <= skill[i] <= 1000
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/
// discuss: https://leetcode.com/problems/divide-players-into-teams-of-equal-skill/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn divide_players(skill: Vec<i32>) -> i64 {
        let mut skill = skill;
        skill.sort_unstable();
        let team_skill = skill.first().unwrap() + skill.last().unwrap();
        let skill_rest = skill.split_off(skill.len() / 2);
        skill
            .into_iter()
            .zip(skill_rest.into_iter().rev())
            .try_fold(0i64, |chemistry, (a, b)| {
                if a + b == team_skill {
                    Ok(chemistry + (a * b) as i64)
                } else {
                    Err(())
                }
            })
            .unwrap_or(-1)
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2491_example_1() {
        let skill = vec![3, 2, 5, 1, 3, 4];

        let result = 22;

        assert_eq!(Solution::divide_players(skill), result);
    }

    #[test]
    fn test_2491_example_2() {
        let skill = vec![3, 4];

        let result = 12;

        assert_eq!(Solution::divide_players(skill), result);
    }

    #[test]
    fn test_2491_example_3() {
        let skill = vec![1, 1, 2, 3];

        let result = -1;

        assert_eq!(Solution::divide_players(skill), result);
    }
}
