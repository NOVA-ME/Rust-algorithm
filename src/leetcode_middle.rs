use std::collections::hash_map::HashMap;

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

// 作者：chang-tiao-ren
// 链接：https://leetcode-cn.com/problems/3sum/solution/rust-jie-jue-wen-ti-by-chang-tiao-ren/
// 来源：力扣（LeetCode）
// 著作权归作者所有。商业转载请联系作者获得授权，非商业转载请注明出处。
pub fn three_num_better(nums: Vec<i32>) -> Vec<Vec<i32>> {
    // 不够三个数, 直接返回空数组
    if nums.len() < 3 {
        return vec![];
    }

    // 先排序，顺序的数组更有利于判断边界条件（是否等于0，最小sum大于0直接进行下一次循环）
    let mut nums = nums.clone();
    nums.sort();

    let mut result = vec![];

    // 遍历, 先选定三数中的第一个数的坐标 a
    let mut a = 0;
    loop {
        // a 最多只能是整个数组里的倒数第三个数
        if a == nums.len() - 2 || nums[a] > 0 {
            break;
        }
        // 本次循环最小sum大于0，直接break循环
        if nums[a] + nums[a + 1] + nums[a + 2] > 0 {
            break;
        }
        if a > 0 && nums[a] == nums[a - 1] {
            //重复的数字， 继续
            a += 1;
            continue;
        }
        // 双指针对撞， 回到两数之和的问题
        let mut b = a + 1;
        let mut c = nums.len() - 1;
        loop {
            if b >= c {
                // 相撞，找不到能和a组合的b，c， 结束循环
                break;
            }

            let sum = nums[a] + nums[b] + nums[c];
            if sum == 0 {
                // 找到了, 加到结果里
                result.push(vec![nums[a], nums[b], nums[c]]);
                // 下面两个while跳过重复的结果
                while b < c && nums[b] == nums[b + 1] {
                    b += 1;
                }
                while b < c && nums[c] == nums[c - 1] {
                    c -= 1;
                }
                // 寻找别的组合
                b += 1;
                c -= 1;
                continue;
            } else if sum < 0 {
                // 太小了, b 👉 移动
                b += 1;
                continue;
            } else {
                // 太大， c 👈 移动
                c -= 1;
                continue;
            }
        }
        a += 1;
    }
    result
}

// 给定一个包含 n 个整数的数组 nums 和一个目标值 target，判断 nums 中是否存在四个元素 a，b，c 和 d ，使得 a + b + c + d 的值与 target 相等？找出所有满足条件且不重复的四元组。

// 注意：
// 答案中不可以包含重复的四元组。

// 示例：
// 给定数组 nums = [1, 0, -1, 0, -2, 2]，和 target = 0。

// 满足要求的四元组集合为：
// [
//   [-1,  0, 0, 1],
//   [-2, -1, 1, 2],
//   [-2,  0, 0, 2]
// ]

// 来源：力扣（LeetCode）
// 链接：https://leetcode-cn.com/problems/4sum
// 自己做出来的，还通过测试，真棒！！！
pub fn four_nums_sum(nums: Vec<i32>, target: i32) -> Vec<Vec<i32>> {
    if nums.len() < 4 {
        return vec![];
    }
    // 将nums排序为增序
    let mut nums = nums.clone();
    nums.sort_unstable();
    let mut res = vec![];
    let (mut a, mut b, mut c, mut d) = (0, 1, 2, nums.len() - 1);
    loop {
        loop {
            // 指针相撞就break
            if c >= d {
                break;
            }
            let sum = nums[a] + nums[b] + nums[c] + nums[d];
            if sum == target {
                res.push(vec![nums[a], nums[b], nums[c], nums[d]]);
                // 去重复
                while c < d && nums[c] == nums[c + 1] {
                    c += 1;
                }
                while d > c && nums[d] == nums[d - 1] {
                    d -= 1;
                }
                c += 1;
                d -= 1;
                continue;
            }

            if sum > target {
                d -= 1;
                continue;
            } else {
                c += 1;
                continue;
            }
        }
        // 去重复
        while b < nums.len() - 3 && nums[b] == nums[b + 1] {
            b += 1;
        }
        while a < b && nums[a] == nums[a + 1] {
            a += 1;
        }
        d = nums.len() - 1;
        if b < nums.len() - 3 {
            b += 1;
            c = b + 1;
        } else if a < nums.len() - 4 {
            a += 1;
            b = a + 1;
            c = b + 1;
        } else {
            break;
        }
    }
    res
}
