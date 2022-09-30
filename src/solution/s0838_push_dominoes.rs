/**
 * [0838] Push Dominoes
 *
 * There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.
 * After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.
 * When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.
 * For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.
 * You are given a string dominoes representing the initial state where:
 *
 * 	dominoes[i] = 'L', if the i^th domino has been pushed to the left,
 * 	dominoes[i] = 'R', if the i^th domino has been pushed to the right, and
 * 	dominoes[i] = '.', if the i^th domino has not been pushed.
 *
 * Return a string representing the final state.
 *  
 * Example 1:
 *
 * Input: dominoes = "RR.L"
 * Output: "RR.L"
 * Explanation: The first domino expends no additional force on the second domino.
 *
 * Example 2:
 * <img alt="" src="https://s3-lc-upload.s3.amazonaws.com/uploads/2018/05/18/domino.png" style="height: 196px; width: 512px;" />
 * Input: dominoes = ".L.R...LR..L.."
 * Output: "LL.RR.LLRRLL.."
 *
 *  
 * Constraints:
 *
 * 	n == dominoes.length
 * 	1 <= n <= 10^5
 * 	dominoes[i] is either 'L', 'R', or '.'.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/push-dominoes/
// discuss: https://leetcode.com/problems/push-dominoes/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/push-dominoes/solutions/1353058/rust-solution/
    pub fn push_dominoes(dominoes: String) -> String {
        let len = dominoes.len();
        let bytes = dominoes.as_bytes();
        let mut v = vec![0; len];
        let mut f = None;
        for i in 0..len {
            f = match bytes[i] {
                b'R' => Some(0),
                b'.' => f.map(|j| j + 1),
                _ => None,
            };
            v[i] -= f.unwrap_or(len as i32);
        }
        for i in (0..len).rev() {
            f = match bytes[i] {
                b'L' => Some(0),
                b'.' => f.map(|j| j + 1),
                _ => None,
            };
            v[i] += f.unwrap_or(len as i32);
        }
        v.iter()
            .map(|f| match f.cmp(&0) {
                std::cmp::Ordering::Less => 'L',
                std::cmp::Ordering::Equal => '.',
                std::cmp::Ordering::Greater => 'R',
            })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0838_example_1() {
        let dominoes = "RR.L".to_string();
        let result = "RR.L".to_string();

        assert_eq!(Solution::push_dominoes(dominoes), result);
    }

    #[test]
    fn test_0838_example_2() {
        let dominoes = ".L.R...LR..L..".to_string();
        let result = "LL.RR.LLRRLL..".to_string();

        assert_eq!(Solution::push_dominoes(dominoes), result);
    }
}
