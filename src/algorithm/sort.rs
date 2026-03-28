/// 堆排序实现

/// 构建最大堆
fn build_max_heap<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    // 从最后一个非叶子节点开始，向前遍历
    for i in (0..len / 2).rev() {
        max_heap(arr, i, len);
    }
}

/// 堆化以i为根的子树
fn max_heap<T: Ord>(arr: &mut [T], i: usize, heap_size: usize) {
    let left = 2 * i + 1;
    let right = 2 * i + 2;
    let mut largest = i;

    if left < heap_size && arr[left] > arr[largest] {
        largest = left;
    }
    if right < heap_size && arr[right] > arr[largest] {
        largest = right;
    }

    if largest != i {
        arr.swap(i, largest);
        max_heap(arr, largest, heap_size);
    }
}

/// 堆排序
pub fn heap_sort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    if len <= 1 {
        return;
    }

    // 构建最大堆
    build_max_heap(arr);

    // 逐个提取堆顶元素到数组末尾
    for i in (1..len).rev() {
        arr.swap(0, i);
        max_heap(arr, 0, i);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_sort() {
        let mut arr = vec![5, 2, 9, 1, 5, 6];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1, 2, 5, 5, 6, 9]);
    }

    #[test]
    fn test_heap_sort_empty() {
        let mut arr: Vec<i32> = vec![];
        heap_sort(&mut arr);
        assert_eq!(arr, Vec::<i32>::new());
    }

    #[test]
    fn test_heap_sort_single() {
        let mut arr = vec![1];
        heap_sort(&mut arr);
        assert_eq!(arr, vec![1]);
    }
}
