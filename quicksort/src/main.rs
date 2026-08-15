fn main() {
    pub use quicksort::quicksort;
    let mut arr: [i64; 20] = [
        90, 900, 90000, 890, 780, 789, 678, 567, 456, 345, 234, 123, 321, 432, 543, 654, 765, 876,
        987, 908,
    ];
    quicksort(&mut arr);
    println!("{:?}", arr);
}
