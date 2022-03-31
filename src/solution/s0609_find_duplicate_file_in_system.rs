/**
 * [0609] Find Duplicate File in System
 *
 * Given a list paths of directory info, including the directory path, and all the files with contents in this directory, return all the duplicate files in the file system in terms of their paths. You may return the answer in any order.
 * A group of duplicate files consists of at least two files that have the same content.
 * A single directory info string in the input list has the following format:
 *
 * 	"root/d1/d2/.../dm f1.txt(f1_content) f2.txt(f2_content) ... fn.txt(fn_content)"
 *
 * It means there are n files (f1.txt, f2.txt ... fn.txt) with content (f1_content, f2_content ... fn_content) respectively in the directory "root/d1/d2/.../dm". Note that n >= 1 and m >= 0. If m = 0, it means the directory is just the root directory.
 * The output is a list of groups of duplicate file paths. For each group, it contains all the file paths of the files that have the same content. A file path is a string that has the following format:
 *
 * 	"directory_path/file_name.txt"
 *
 *  
 * Example 1:
 * Input: paths = ["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)","root 4.txt(efgh)"]
 * Output: [["root/a/2.txt","root/c/d/4.txt","root/4.txt"],["root/a/1.txt","root/c/3.txt"]]
 * Example 2:
 * Input: paths = ["root/a 1.txt(abcd) 2.txt(efgh)","root/c 3.txt(abcd)","root/c/d 4.txt(efgh)"]
 * Output: [["root/a/2.txt","root/c/d/4.txt"],["root/a/1.txt","root/c/3.txt"]]
 *  
 * Constraints:
 *
 * 	1 <= paths.length <= 2 * 10^4
 * 	1 <= paths[i].length <= 3000
 * 	1 <= sum(paths[i].length) <= 5 * 10^5
 * 	paths[i] consist of English letters, digits, '/', '.', '(', ')', and ' '.
 * 	You may assume no files or directories share the same name in the same directory.
 * 	You may assume each given directory info represents a unique directory. A single blank space separates the directory path and file info.
 *
 *  
 * Follow up:
 *
 * 	Imagine you are given a real file system, how will you search files? DFS or BFS?
 * 	If the file content is very large (GB level), how will you modify your solution?
 * 	If you can only read the file by 1kb each time, how will you modify your solution?
 * 	What is the time complexity of your modified solution? What is the most time-consuming part and memory-consuming part of it? How to optimize?
 * 	How to make sure the duplicated files you find are not false positive?
 *
 */
pub struct Solution {}

// problem: https://leetcode.com/problems/find-duplicate-file-in-system/
// discuss: https://leetcode.com/problems/find-duplicate-file-in-system/discuss/?currentPage=1&orderBy=most_votes&query=

// submission codes start here

impl Solution {
    pub fn find_duplicate(paths: Vec<String>) -> Vec<Vec<String>> {
        let mut hm = std::collections::HashMap::new();
        for path in &paths {
            let files = path.split(' ').collect::<Vec<_>>();
            for &file in &files[1..] {
                if let Some(pos) = file.chars().position(|c| c == '(') {
                    hm.entry(&file[pos + 1..file.len() - 1])
                        .or_insert_with(Vec::new)
                        .push(String::new() + files[0] + "/" + &file[..pos]);
                }
            }
        }

        hm.into_iter()
            .filter_map(|(_, v)| if v.len() > 1 { Some(v) } else { None })
            .collect()
    }
}

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0609_example_1() {
        let paths = vec_string![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)",
            "root 4.txt(efgh)"
        ];
        let result = vec![
            vec_string!["root/a/2.txt", "root/c/d/4.txt", "root/4.txt"],
            vec_string!["root/a/1.txt", "root/c/3.txt"],
        ];

        assert_eq_sorted!(Solution::find_duplicate(paths), result);
    }

    #[test]
    fn test_0609_example_2() {
        let paths = vec_string![
            "root/a 1.txt(abcd) 2.txt(efgh)",
            "root/c 3.txt(abcd)",
            "root/c/d 4.txt(efgh)"
        ];
        let result = vec![
            vec_string!["root/a/2.txt", "root/c/d/4.txt"],
            vec_string!["root/a/1.txt", "root/c/3.txt"],
        ];

        assert_eq_sorted!(Solution::find_duplicate(paths), result);
    }
}
