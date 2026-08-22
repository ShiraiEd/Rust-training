//Heap sort implementation
//

// well this one actually sorts, but it doesnt heapfy correctyly
//
fn heapsort(arr: &mut [i32]) {
    if arr.len() <= 1 {
        return;
    }
    let mut last = arr.len() - 1;
    while last > 0 {
        heapfymax(arr, last);
        arr.swap(0, last);
        last -= 1;
    }
}
fn heapfymax(arr: &mut [i32], last: usize) {
    let mut last = last;
    if last % 2 == 0 {
        if arr[last - 1] >= arr[last] && arr[last - 1] > arr[(last - 1) / 2] {
            arr.swap(last - 1, (last - 1) / 2);
        } else if arr[last] > arr[(last - 1) / 2] {
            arr.swap(last, (last - 1) / 2);
        }
        last -= 2;
    } else {
        if arr[last] > arr[last / 2] {
            arr.swap(last, last / 2);
        }
        last -= 1;
    }
    if last > 0 {
        heapfymax(arr, last);
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn works_with_simple_list() {
        let mut test = vec![3, 5, 4];
        heapsort(&mut test);
        assert_eq!(vec![3, 4, 5,], test);
    }
    #[test]
    fn works_with_empty_list() {
        let mut test = Vec::<i32>::new();
        heapsort(&mut test);
        assert!(test.is_empty());
    }
    #[test]
    fn works_with_singular_unit_list() {
        let mut test = vec![4];
        heapsort(&mut test);
        assert_eq!(vec![4], test);
    }
    #[test]
    fn works_with_negative_ints_list() {
        let mut test = vec![-3, -5, -4];
        heapsort(&mut test);
        assert_eq!(vec![-5, -4, -3], test);
    }
    #[test]
    fn works_with_mixed_list() {
        let mut test = vec![3, 5, 4, 0, -10, -5, 100];
        heapsort(&mut test);
        assert_eq!(vec![-10, -5, 0, 3, 4, 5, 100], test);
    }
}
