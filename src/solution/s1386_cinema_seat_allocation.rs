/**
 * [1386] Cinema Seat Allocation
 *
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_1.png" style="width: 400px; height: 149px;" />
 * A cinema has n rows of seats, numbered from 1 to n and there are ten seats in each row, labelled from 1 to 10 as shown in the figure above.
 * Given the array reservedSeats containing the numbers of seats already reserved, for example, reservedSeats[i] = [3,8] means the seat located in row 3 and labelled with 8 is already reserved.
 * Return the maximum number of four-person groups you can assign on the cinema seats. A four-person group occupies four adjacent seats in one single row. Seats across an aisle (such as [3,3] and [3,4]) are not considered to be adjacent, but there is an exceptional case on which an aisle split a four-person group, in that case, the aisle split a four-person group in the middle, which means to have two people on each side.
 *  
 * Example 1:
 * <img alt="" src="https://assets.leetcode.com/uploads/2020/02/14/cinema_seats_3.png" style="width: 400px; height: 96px;" />
 *
 * Input: n = 3, reservedSeats = [[1,2],[1,3],[1,8],[2,6],[3,1],[3,10]]
 * Output: 4
 * Explanation: The figure above shows the optimal allocation for four groups, where seats mark with blue are already reserved and contiguous seats mark with orange are for one group.
 *
 * Example 2:
 *
 * Input: n = 2, reservedSeats = [[2,1],[1,8],[2,6]]
 * Output: 2
 *
 * Example 3:
 *
 * Input: n = 4, reservedSeats = [[4,3],[1,4],[4,6],[1,7]]
 * Output: 4
 *
 *  
 * Constraints:
 *
 * 	1 <= n <= 10^9
 * 	1 <= reservedSeats.length <= min(10*n, 10^4)
 * 	reservedSeats[i].length == 2
 * 	1 <= reservedSeats[i][0] <= n
 * 	1 <= reservedSeats[i][1] <= 10
 * 	All reservedSeats[i] are distinct.
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/cinema-seat-allocation/
// discuss: https://leetcode.com/problems/cinema-seat-allocation/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn max_number_of_families(n: i32, reserved_seats: Vec<Vec<i32>>) -> i32 {
        let mut seats = std::collections::HashMap::new();

        const PADDING: i32 = i32::MAX << 10;
        const FAM: i32 = (1 << 4) - 1;
        for s in reserved_seats {
            let row = s[0];
            let occupied = s[1];
            *seats.entry(row as usize - 1).or_insert(0) |= (PADDING) | (1 << (10 - occupied));
        }
        seats.iter_mut().for_each(|(_, v)| *v = PADDING | !*v);

        const FAM_RIGHT: i32 = PADDING | FAM << 1;
        const FAM_LEFT: i32 = PADDING | FAM << 5;
        const FAM_MID: i32 = PADDING | FAM << 3;
        const FAM_TWO: i32 = FAM_LEFT | FAM_RIGHT;

        let mut result = 0;
        for s in seats.values() {
            if FAM_TWO & s == FAM_TWO {
                result += 2;
            } else if (FAM_RIGHT & s == FAM_RIGHT)
                || (FAM_LEFT & s == FAM_LEFT)
                || (FAM_MID & s == FAM_MID)
            {
                result += 1
            }
        }
        result + (n - seats.len() as i32) * 2
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1386_example_1() {
        let n = 3;
        let reserved_seats = vec![
            vec![1, 2],
            vec![1, 3],
            vec![1, 8],
            vec![2, 6],
            vec![3, 1],
            vec![3, 10],
        ];

        let result = 4;

        assert_eq!(Solution::max_number_of_families(n, reserved_seats), result);
    }

    #[test]
    fn test_1386_example_2() {
        let n = 2;
        let reserved_seats = vec![vec![2, 1], vec![1, 8], vec![2, 6]];

        let result = 2;

        assert_eq!(Solution::max_number_of_families(n, reserved_seats), result);
    }

    #[test]
    fn test_1386_example_3() {
        let n = 4;
        let reserved_seats = vec![vec![4, 3], vec![1, 4], vec![4, 6], vec![1, 7]];

        let result = 4;

        assert_eq!(Solution::max_number_of_families(n, reserved_seats), result);
    }
}
