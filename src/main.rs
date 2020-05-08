mod leetcode_simple;
mod sort;
fn main() {
    let num_ary = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3];
    sort::bubble_sort(&mut num_ary.clone());
    sort::select_sort(&mut num_ary.clone());
    leetcode_simple::two_sum(&num_ary.clone(), 12);
    leetcode_simple::two_sum_map_solve(&num_ary.clone(), 12);
    leetcode_simple::reverse_num(999999999999999999);
    leetcode_simple::reverse_num(1234000);
}
