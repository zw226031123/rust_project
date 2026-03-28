use rust_project::algorithm::sort;

fn main() {
    let mut arr = vec![5, 2, 9, 1, 5, 6];
    sort::heap_sort(&mut arr);
    assert_eq!(arr, vec![1, 2, 5, 5, 6, 9]);
}
