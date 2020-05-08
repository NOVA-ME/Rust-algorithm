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
