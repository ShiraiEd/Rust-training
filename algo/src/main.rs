fn main() {
    let arr = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let arr2: [i32; 10] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    for i in arr2 {
        match linear_search(&arr2, &i) {
            Some(posicao) => println!("Linear: item da array: {i}\nPosicao: {}", posicao),
            None => println!("Item ao encontrado na array"),
        }
        println!(" ");
    }

    println!("-------------------");
    println!(" ");

    for i in &arr {
        match binary_search(&arr, i) {
            Some(posicao) => println!("Binary: item da array: {i}\nPosicao: {}", posicao),
            None => println!("Item nao encontrado na array"),
        }
        println!(" ");
    }
}

fn linear_search(arr: &[i32], item: &i32) -> Option<isize> {
    for (idx, valor) in arr.iter().enumerate() {
        if valor == item {
            return Some(idx as isize);
        }
    }
    None
}

fn binary_search(arr: &[i32], item: &i32) -> Option<i32> {
    let metade = ((arr.len() as f64 / 2_f64).ceil() - 1_f64) as usize;
    match arr[metade].cmp(item) {
        std::cmp::Ordering::Equal => Some(metade as i32),

        std::cmp::Ordering::Less => {
            let slice = &arr[metade + 1..];
            Some(binary_search(slice, item).unwrap() + metade as i32 + 1)
        }
        std::cmp::Ordering::Greater => {
            let slice = &arr[..metade];
            binary_search(slice, item)
        }
    }
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
}
