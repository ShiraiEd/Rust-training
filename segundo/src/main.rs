fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arr2: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    print_resultado("Linear com Vec", &arr, linear_search);
    print_resultado("Linear com array", &arr2, linear_search);
    print_resultado("Binary com Vec", &arr, binary_search);
    print_resultado("Binary com array", &arr2, binary_search);
}
fn print_resultado<F>(name: &str, array: &[i32], search: F)
where
    F: Fn(&[i32], &i32) -> Option<usize>,
{
    for i in array {
        match search(array, i) {
            Some(posicao) => println!("{name}: item da array: {i}\nPosicao: {posicao}"),
            None => println!("{name}: Item {i} nao encontrado na array"),
        }
        println!();
    }

    println!("-------------------");
    println!();
}

fn linear_search(arr: &[i32], item: &i32) -> Option<usize> {
    for (idx, valor) in arr.iter().enumerate() {
        if valor == item {
            return Some(idx);
        }
    }
    None
}

fn binary_search(arr: &[i32], item: &i32) -> Option<usize> {
    let metade = ((arr.len() as f64 / 2_f64).ceil() - 1_f64) as usize;
    match arr[metade].cmp(item) {
        std::cmp::Ordering::Equal => Some(metade),

        std::cmp::Ordering::Less => {
            let slice = &arr[metade + 1..];
            Some(binary_search(slice, item)? + metade + 1)
        }
        std::cmp::Ordering::Greater => {
            let slice = &arr[..metade];
            binary_search(slice, item)
        }
    }
}
