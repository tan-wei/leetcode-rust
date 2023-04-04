/**
 * [1024] Video Stitching
 *
 * You are given a series of video clips from a sporting event that lasted time seconds. These video clips can be overlapping with each other and have varying lengths.
 * Each video clip is described by an array clips where clips[i] = [starti, endi] indicates that the ith clip started at starti and ended at endi.
 * We can cut these clips into segments freely.
 *
 * 	For example, a clip [0, 7] can be cut into segments [0, 1] + [1, 3] + [3, 7].
 *
 * Return the minimum number of clips needed so that we can cut the clips into segments that cover the entire sporting event [0, time]. If the task is impossible, return -1.
 *  
 * Example 1:
 *
 * Input: clips = [[0,2],[4,6],[8,10],[1,9],[1,5],[5,9]], time = 10
 * Output: 3
 * Explanation: We take the clips [0,2], [8,10], [1,9]; a total of 3 clips.
 * Then, we can reconstruct the sporting event as follows:
 * We cut [1,9] into segments [1,2] + [2,8] + [8,9].
 * Now we have segments [0,2] + [2,8] + [8,10] which cover the sporting event [0, 10].
 *
 * Example 2:
 *
 * Input: clips = [[0,1],[1,2]], time = 5
 * Output: -1
 * Explanation: We cannot cover [0,5] with only [0,1] and [1,2].
 *
 * Example 3:
 *
 * Input: clips = [[0,1],[6,8],[0,2],[5,6],[0,4],[0,3],[6,7],[1,3],[4,7],[1,4],[2,5],[2,6],[3,4],[4,5],[5,7],[6,9]], time = 9
 * Output: 3
 * Explanation: We can take clips [0,4], [4,7], and [6,9].
 *
 *  
 * Constraints:
 *
 * 	1 <= clips.length <= 100
 * 	0 <= starti <= endi <= 100
 * 	1 <= time <= 100
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/video-stitching/
// discuss: https://leetcode.com/problems/video-stitching/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn video_stitching(clips: Vec<Vec<i32>>, time: i32) -> i32 {
        let mut min = 0;
        let mut max = 0;
        let mut total = 0;
        while max < time {
            for i in 0..clips.len() {
                let l = clips[i][0];
                let r = clips[i][1];
                if l <= min && r > max {
                    max = r;
                }
            }
            if min == max {
                return -1;
            };
            min = max;
            total += 1;
        }
        total
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1024_example_1() {
        let clips = vec![
            vec![0, 2],
            vec![4, 6],
            vec![8, 10],
            vec![1, 9],
            vec![1, 5],
            vec![5, 9],
        ];
        let time = 10;
        let result = 3;

        assert_eq!(Solution::video_stitching(clips, time), result);
    }

    #[test]
    fn test_1024_example_2() {
        let clips = vec![vec![0, 1], vec![1, 2]];
        let time = 5;
        let result = -1;

        assert_eq!(Solution::video_stitching(clips, time), result);
    }

    #[test]
    fn test_1024_example_3() {
        let clips = vec![
            vec![0, 1],
            vec![6, 8],
            vec![0, 2],
            vec![5, 6],
            vec![0, 4],
            vec![0, 3],
            vec![6, 7],
            vec![1, 3],
            vec![4, 7],
            vec![1, 4],
            vec![2, 5],
            vec![2, 6],
            vec![3, 4],
            vec![4, 5],
            vec![5, 7],
            vec![6, 9],
        ];
        let time = 9;
        let result = 3;

        assert_eq!(Solution::video_stitching(clips, time), result);
    }
}
