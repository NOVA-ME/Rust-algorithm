use std::time::SystemTime;

mod leetcode_middle;
mod leetcode_simple;
mod sort;
fn main() {
    let num_ary = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3];
    let three_num_test = vec![-1, 0, 1, 2, -1, -4];
    let str_ary = vec![
        "pwwkew", "abcabcbb", "aaaaaaa", "ausd", " ", "aab", "cdd", "abba", "dvdf", "abbbc",
        "tmmzuxt", "abbcsef",
    ];
    sort::bubble_sort(&mut num_ary.clone());
    sort::select_sort(&mut num_ary.clone());
    leetcode_simple::two_sum(&num_ary.clone(), 12);
    leetcode_simple::two_sum_map_solve(&num_ary.clone(), 12);
    leetcode_simple::reverse_num(999999999999999999);
    leetcode_simple::reverse_num(1234000);
    let start = SystemTime::now();
    let mut values = Vec::new();
    for i in str_ary.iter() {
        values.push(leetcode_middle::longest_sub_str(String::from(*i)));
    }
    let end = SystemTime::now();
    match end.duration_since(start) {
        Ok(spend) => println!(
            "自己的算法 结果：{:?}，耗时：{} 微秒",
            values,
            spend.as_micros()
        ),
        Err(_) => panic!("时间错误"),
    }
    values.clear();
    let start = SystemTime::now();

    for i in str_ary.iter() {
        values.push(leetcode_middle::longest_sub_str_better(String::from(*i)));
    }
    let end = SystemTime::now();
    match end.duration_since(start) {
        Ok(spend) => println!(
            "网友的算法 结果：{:?}，耗时：{} 微秒",
            values,
            spend.as_micros()
        ),
        Err(_) => panic!("时间错误 "),
    }
    leetcode_middle::three_num(three_num_test);
    let nums = vec![-5,-4,-3,-2,-1,0,0,1,2,3,4,5];
    let res = leetcode_middle::four_nums_sum(nums, 0);
    println!("{:?}", res);
}
