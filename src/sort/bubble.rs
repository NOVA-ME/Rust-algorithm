pub fn bubble_sort(ary: &mut Vec<i32>) {
    println!("未经过冒泡排序{:?}", ary);
    for i in 0..ary.len() {
        for j in (i + 1)..ary.len() {
            if ary[i] > ary[j] {
                ary.swap(j, i);
            }
        }
    }
    println!("经过冒泡排序后的{:?}", ary);
}
