/**
 * [0514] Freedom Trail
 *
 * In the video game Fallout 4, the quest "Road to Freedom" requires players to reach a metal dial called the "Freedom Trail Ring" and use the dial to spell a specific keyword to open the door.
 * Given a string ring that represents the code engraved on the outer ring and another string key that represents the keyword that needs to be spelled, return the minimum number of steps to spell all the characters in the keyword.
 * Initially, the first character of the ring is aligned at the "12:00" direction. You should spell all the characters in key one by one by rotating ring clockwise or anticlockwise to make each character of the string key aligned at the "12:00" direction and then by pressing the center button.
 * At the stage of rotating the ring to spell the key character key[i]:
 * <ol>
 * 	You can rotate the ring clockwise or anticlockwise by one place, which counts as one step. The final purpose of the rotation is to align one of ring's characters at the "12:00" direction, where this character must equal key[i].
 * 	If the character key[i] has been aligned at the "12:00" direction, press the center button to spell, which also counts as one step. After the pressing, you could begin to spell the next character in the key (next stage). Otherwise, you have finished all the spelling.
 * </ol>
 *  
 * Example 1:
 * <img src="https://assets.leetcode.com/uploads/2018/10/22/ring.jpg" style="width: 450px; height: 450px;" />
 * Input: ring = "godding", key = "gd"
 * Output: 4
 * Explanation:
 * For the first key character 'g', since it is already in place, we just need 1 step to spell this character.
 * For the second key character 'd', we need to rotate the ring "godding" anticlockwise by two steps to make it become "ddinggo".
 * Also, we need 1 more step for spelling.
 * So the final output is 4.
 *
 * Example 2:
 *
 * Input: ring = "godding", key = "godding"
 * Output: 13
 *
 *  
 * Constraints:
 *
 * 	1 <= ring.length, key.length <= 100
 * 	ring and key consist of only lower case English letters.
 * 	It is guaranteed that key could always be spelled by rotating ring.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/freedom-trail/
// discuss: https://leetcode.com/problems/freedom-trail/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    // Credit: https://leetcode.com/problems/freedom-trail/discuss/231829/Annotated-DP-solution-using-Rust.
    pub fn find_rotate_steps(ring: String, key: String) -> i32 {
        let mut index = vec![Vec::new(); 26]; // indices of each ring character
        for (idx, char) in ring.chars().enumerate() {
            index[(char as u8 - 'a' as u8) as usize].push(idx);
        }
        // d[i][j] = min steps to rotate key[j..] with ring pointing at index i
        let mut d = vec![vec![0; key.len() + 1]; ring.len()];
        for k_pos in (0..key.len()).rev() {
            for r_pos in (0..ring.len()) {
                let mut min_so_far = std::i32::MAX;
                for r_idx in &index[(key.as_bytes()[k_pos] - 'a' as u8) as usize] {
                    // find the clockwise rotation from current position r_pos to
                    // position r_idx pointing at the desired character key[k_pos]
                    let distance = (r_pos as i32 - *r_idx as i32).abs();
                    // check if counter-clockwise rotation is shorter
                    let min_distance = std::cmp::min(distance, ring.len() as i32 - distance);
                    // relax min_so_far in case rotating to r_idx is beneficial
                    min_so_far = std::cmp::min(min_so_far, min_distance + d[*r_idx][k_pos + 1])
                }
                d[r_pos][k_pos] = min_so_far + 1; // steps to rotate + pressing the center button
            }
        }
        d[0][0] // min steps to rotate k[0..] with ring at its initial position 0
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0514_example_1() {
        let ring = "godding".to_string();
        let key = "gd".to_string();
        let result = 4;

        assert_eq!(Solution::find_rotate_steps(ring, key), result);
    }

    #[test]
    fn test_0514_example_2() {
        let ring = "godding".to_string();
        let key = "godding".to_string();
        let result = 13;

        assert_eq!(Solution::find_rotate_steps(ring, key), result);
    }
}
