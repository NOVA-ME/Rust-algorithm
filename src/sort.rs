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

pub fn select_sort(ary: &mut Vec<i32>) {
    println!("未经过选择排序{:?}", ary);
    for i in 0..ary.len() {
        let mut min_idx = i;
        for j in (i + 1)..ary.len() {
            if ary[j] < ary[min_idx] {
                min_idx = j;
            }
        }
        ary.swap(min_idx, i);
    }
    println!("经过选择排序后的{:?}", ary);
}
