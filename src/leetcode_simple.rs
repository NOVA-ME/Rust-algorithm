use std::collections::hash_map::HashMap;
use std::time::SystemTime;

// 给定一个整数数组 nums 和一个目标值 target，请你在该数组中找出和为目标值的那 两个 整数，并返回他们的数组下标。
// 你可以假设每种输入只会对应一个答案。但是，数组中同一个元素不能使用两遍。
// 示例:
// 给定 nums = [2, 7, 11, 15], target = 9
// 因为 nums[0] + nums[1] = 2 + 7 = 9
// 所以返回 [0, 1]

// 时间复杂度O(n*n) 空间复杂度O(1)
pub fn two_sum(ary: &Vec<i32>, target: i32) -> Vec<i32> {
    let length = ary.len();
    for i in 0..length {
        for j in 0..length {
            if j == i {
                continue;
            }
            if target == ary[j] + ary[i] {
                println!("符合条件的下标{}---{}", i, j);
                return vec![i as i32, j as i32];
            }
        }
    }
    panic!("未找到符合条件的下标");
}

// 时间复杂度O(n) 空间复杂度O(n)
pub fn two_sum_map_solve(ary: &Vec<i32>, target: i32) -> Vec<i32> {
    println!("运行了");
    let mut map: HashMap<i32, i32> = HashMap::new();
    for (i, v) in ary.iter().enumerate() {
        let complement = target - *v;
        if map.contains_key(&complement) {
            if let Some(&a) = map.get(&complement) {
                println!("符合条件的下标{}---{}", i, a);
                return vec![i as i32, a];
            }
        }
        map.insert(*v, i as i32);
    }
    panic!("未找到符合条件的下标");
}

// 给出一个 32 位的有符号整数，你需要将这个整数中每位上的数字进行反转。

// 示例 1:
// 输入: 123
// 输出: 321

//  示例 2:
// 输入: -123
// 输出: -321

// 示例 3:
// 输入: 120
// 输出: 21
// 注意:

// 假设我们的环境只能存储得下 32 位的有符号整数，则其数值范围为 [−231,  231 − 1]。请根据这个假设，如果反转后整数溢出那么就返回 0。
pub fn reverse_num(i: i64) -> i32 {
    let start = SystemTime::now();
    println!("{}", i);
    let mut a = i;
    let mut num = 0;
    loop {
        num = a % 10 + num * 10;
        a = a / 10;
        if a == 0 {
            let end = SystemTime::now();
            match end.duration_since(start) {
                Ok(n) => println!("{} 耗时{}", num, n.as_micros()),
                Err(_) => panic!("时间错误"),
            }
            return num as i32;
        }
    }
}

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
