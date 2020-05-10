use std::collections::hash_map::HashMap;
use std::time::SystemTime;

// 给定一个字符串，请你找出其中不含有重复字符的 最长子串 的长度。

// 示例 1:
// 输入: "abcabcbb"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "abc"，所以其长度为 3。

// 示例 2:
// 输入: "bbbbb"
// 输出: 1
// 解释: 因为无重复字符的最长子串是 "b"，所以其长度为 1。

// 示例 3:
// 输入: "pwwkew"
// 输出: 3
// 解释: 因为无重复字符的最长子串是 "wke"，所以其长度为 3。
//      请注意，你的答案必须是 子串 的长度，"pwke" 是一个子序列，不是子串。

pub fn longest_sub_str(s: String) -> i32 {
    if s == "" {
        return 0;
    }
    if s.len() <= 1 {
        return 1;
    }
    let mut map: HashMap<char, usize> = HashMap::new();
    let chars = s.chars();
    let mut max_len = 0;
    let mut cursor_start = 0;
    let mut len;
    for (i, v) in chars.into_iter().enumerate() {
        let cursor_end = i as i32;
        let idx;
        match map.get(&v) {
            Some(&t) => idx = t as i32,
            None => idx = -1,
        }
        if idx >= cursor_start {
            len = cursor_end - cursor_start;
            cursor_start = idx as i32 + 1;
        } else {
            len = cursor_end - cursor_start + 1;
        }
        max_len = std::cmp::max(len, max_len);
        map.insert(v, i);
    }
    max_len
}

// leetcode 网友更好的答案
// 作者：icespark
// 链接：https://leetcode-cn.com/problems/longest-substring-without-repeating-characters/solution/rust-hua-dong-chuang-kou-tong-pai-xu-by-icespark/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn longest_sub_str_better(s: String) -> i32 {
    if &s == "" {
        return 0 as i32;
    }
    let s = &s.trim();
    let (mut str_s, mut str_e, mut length, mut result) = (0, 0, 0, 0);
    let mut char_map: Vec<i32> = vec![-1; 128];
    let str_len = s.len();
    if str_len <= 1 {
        return 1 as i32;
    }
    while str_e < str_len {
        let mut cur_char = '0';
        // .chars将&str转化为Chars(chars的迭代器)
        for c in s[str_e..str_e + 1].chars() {
            cur_char = c;
        }
        let cur_char_index = cur_char as usize - 0 as usize;
        // println!("The current char is {}, converted index is {}.", cur_char, cur_char_index);
        if char_map[cur_char_index] >= str_s as i32 {
            str_s = (char_map[cur_char_index] + 1) as usize;
            length = (str_e - str_s) as i32;
        }
        char_map[cur_char_index] = str_e as i32;
        length += 1;
        str_e += 1;
        result = std::cmp::max(length, result);
        // println!("The str_e is {}, length is {}, result is {}.", str_e, length, result);
    }
    result
}

// 给你一个包含 n 个整数的数组 nums，判断 nums 中是否存在三个元素 a，b，c ，使得 a + b + c = 0 ？请你找出所有满足条件且不重复的三元组。
// 注意：答案中不可以包含重复的三元组。
//

// 示例：
// 给定数组 nums = [-1, 0, 1, 2, -1, -4]，
// 满足要求的三元组集合为：
// [
//   [-1, 0, 1],
//   [-1, -1, 2]
// ]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/3sum

pub fn three_num(nums: Vec<i32>) -> Vec<Vec<i32>> {
    let mut nums = nums.clone();
    if nums.len() < 3 {
        let v: Vec<Vec<i32>> = Vec::new();
        return v;
    }
    nums.sort();
    let min = nums[0];
    let mut result = vec![];
    for s in 0..nums.len() {
        for i in s + 1..nums.len() {
            let sum_two = nums[s] + nums[i];
            if sum_two + min > 0 {
                break;
            }
            for j in i + 1..nums.len() {
                if sum_two + nums[j] == 0 {
                    let mut v = vec![nums[s], nums[i], nums[j]];
                    v.sort();
                    if !result.contains(&v) {
                        result.push(v);
                    }
                }
            }
        }
    }
    println!("{:?}", result);
    result
}
