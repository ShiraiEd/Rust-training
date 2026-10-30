//Heap sort implementation
//
// well this one actually sorts, but it doesnt heapfy correctyly
//
//
fn heapsort(arr: &mut [i32]) {
    let len = arr.len();
    const BASE: f64 = 2.0;
    let len = len as f64;
    let levels = len.log(BASE);
    let rep = len.log(BASE);
    let mut idxf = 0.0;
    for n in 0..(levels as i32 - 1) {
        let n = n as f64;
        idxf += n.log(BASE);
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
