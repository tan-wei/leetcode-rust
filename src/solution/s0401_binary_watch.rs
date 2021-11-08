/**
 * [0401] Binary Watch
 *
 * A binary watch has 4 LEDs on the top which represent the hours (0-11), and the 6 LEDs on the bottom represent the minutes (0-59). Each LED represents a zero or one, with the least significant bit on the right.
 *
 * 	For example, the below binary watch reads "4:51".
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2021/04/08/binarywatch.jpg" style="width: 500px; height: 500px;" />
 * Given an integer turnedOn which represents the number of LEDs that are currently on, return all possible times the watch could represent. You may return the answer in any order.
 * The hour must not contain a leading zero.
 *
 * 	For example, "01:00" is not valid. It should be "1:00".
 *
 * The minute must be consist of two digits and may contain a leading zero.
 *
 * 	For example, "10:2" is not valid. It should be "10:02".
 *
 *  
 * Example 1:
 * Input: turnedOn = 1
 * Output: ["0:01","0:02","0:04","0:08","0:16","0:32","1:00","2:00","4:00","8:00"]
 * Example 2:
 * Input: turnedOn = 9
 * Output: []
 *  
 * Constraints:
 *
 * 	0 <= turnedOn <= 10
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/binary-watch/
// discuss: https://leetcode.com/problems/binary-watch/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn read_binary_watch(turned_on: i32) -> Vec<String> {
        let hours = vec![
            vec!["0"],
            vec!["1", "2", "4", "8"],
            vec!["3", "5", "6", "9", "10"],
            vec!["7", "11"],
        ];
        let mut minutes = vec![Vec::<String>::new(); 6];
        let count_set_bits = |x: i32| -> usize {
            let mut x = x;
            let mut c = 0;
            while x > 0 {
                x &= x - 1;
                c += 1;
            }
            c
        };
        for i in 0..60 {
            let set_bits = count_set_bits(i);
            let minutes = &mut minutes[set_bits];
            minutes.push(if i < 10 {
                format!("0{}", i)
            } else {
                format!("{}", i)
            });
        }

        let num = turned_on as usize;
        let mut times = Vec::new();
        let max_hour_bits = std::cmp::min(num, 3);
        for x in 0..max_hour_bits + 1 {
            let minute_bits = num - x;
            if minute_bits < 6 {
                for hour in &hours[x] {
                    for minute in &minutes[minute_bits] {
                        times.push(format!("{}:{}", hour, minute));
                    }
                }
            }
        }
        times
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0401_example_1() {
        let turned_on = 1;
        let result = vec_string![
            "0:01", "0:02", "0:04", "0:08", "0:16", "0:32", "1:00", "2:00", "4:00", "8:00"
        ];

        assert_eq!(Solution::read_binary_watch(turned_on), result);
    }

    #[test]
    fn test_0401_example_2() {
        let turned_on = 9;
        let result: Vec<String> = vec![];

        assert_eq!(Solution::read_binary_watch(turned_on), result);
    }
}
