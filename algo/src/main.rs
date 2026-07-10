fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arr2: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    println!("primeira busca l: {}", linear_search(&arr, 6));
    println!("segunda busca l: {}", linear_search(&arr, 8));
    println!("terceira busca l: {}", linear_search(&arr, 11));
    // println!("busca binaria: {}", binary_search(&arr, 1));
    // println!("segunda busca b: {}", binary_search(&arr, 2));
    // println!("terceira busca b: {}", binary_search(&arr, 3));
    // println!("quarta busca b: {}", binary_search(&arr, 6));
}

fn linear_search(arr: &[i32], item: i32) -> isize {
    for (idx, valor) in arr.iter().enumerate() {
        if *valor == item {
            return idx as isize;
        }
    }
    -1
}

// fn binary_search(arr: &[i32], item: i32) -> i32 {
//     let mut metade = (arr.len() as f64 / 2_f64).round() as usize;
//     while arr[metade] != item {
//         println!("index: {metade}");
//         println!("valor: {}", arr[metade]);
//         if arr[metade] > item {
//             metade = (metade as f64 - (metade as f64 / 2_f64).round()).round() as usize;
//         } else {
//             metade = (((arr.len() as f64) - metade as f64) / 2_f64).ceil() as usize + metade;
//         }
//     }
//     metade as i32
// }
