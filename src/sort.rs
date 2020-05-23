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

pub fn insert_sort(ary: &mut Vec<i32>) {
    println!("未经过插入排序 {:?}", ary);
    // 插入排序假定前面是一个有序的数组a，这里假定第一个数字是有序的数组a
    for i in 1..ary.len() {
        let temp = ary[i];
        let mut j = i;
        while j > 0 && temp < ary[j - 1] {
            ary[j] = ary[j - 1];
            j -= 1;
        }
        ary[j] = temp;
    }
    println!("经过插入排序后 {:?}", ary);
}
