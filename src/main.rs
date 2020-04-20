mod sort;
use sort::bubble;

fn main() {
    let mut num_ary = vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0, 1, 2, 3];
    bubble::bubble_sort(&mut num_ary);
}
