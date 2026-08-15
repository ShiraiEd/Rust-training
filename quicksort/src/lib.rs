/// Treino: quicksort in-place.
///
/// Assinatura esperada pelos testes: `pub fn quicksort<T: Ord>(arr: &mut [T])`
///
/// Repare que NAO tem `Copy` no bound e nao tem valor de retorno: a ordenacao
/// acontece dentro do slice que voce recebeu emprestado. Isso muda tudo em
/// relacao ao merge sort — voce nao pode `push` num `Vec` novo, tem que trocar
/// os elementos de lugar.
///
/// Duas ferramentas que voce vai precisar (e que o merge sort nao pediu):
///   - `arr.swap(i, j)` — troca dois elementos sem mover nada pra fora do slice
///   - `arr.split_at_mut(meio)` — a UNICA forma de ter dois `&mut` apontando
///     pro mesmo slice ao mesmo tempo (o borrow checker aceita porque as duas
///     metades nao se sobrepoem)
///
/// Rode `cargo test` — vai falhar na compilacao ate a funcao existir. É o ponto
/// de partida (vermelho).
// [2, 8, 7, 1, 3, 5, 6, 4]
// i,j = 0
// 2 < 4
// swap 0,0
// i,j +1
//
// i,j = 1
// 8 > 4
// nada
// j+1
//
// i = 1
// j = 2
// 7 > 4
// nada
// j+1
//
// i =1
// j = 3
// 1 < 4
// swap 1,3
//[2, 1, 7, 8, 3, 5, 6, 4]
//i + 1
//j + 1
//
//i = 2
//j = 4
//3 < 4
//swap 2, 4
//[2, 1, 3, 8, 7, 5, 6, 4]
//i+1
//j + 1
//
//i=3
//j = 5
//5 > 4
//nada
//j + 1
//
//i = 3
//j = 6
//6 > 4
//nada
//j + 1
//
//i = 3
//j = 7
//j = final array
//swap 3, 7
//[2, 1, 3, 4, 7, 5, 6, 8]
//
//split_at_mut no i ,
//faz recursivamente dos dois lados, excluindo i
//ate a array retornar apenas 1 numero(base case)
pub fn quicksort<T: Ord>(arr: &mut [T]) {
    if arr.is_empty() {
        return;
    }
    if arr.len() <= 1 {
        return;
    }
    let last = arr.len() - 1;
    let mut meio = 0;

    for n in 0..last {
        if arr[n] >= arr[last] {
        } else if arr[n] < arr[last] {
            arr.swap(meio, n);
            meio += 1;
        }
    }
    arr.swap(meio, last);
    let (esquerda, direita) = arr.split_at_mut(meio);
    let direita = &mut direita[1..];
    quicksort(esquerda);
    quicksort(direita);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_lista_positiva() {
        let mut arr = [8, 7, 3, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [3, 5, 7, 8]);
    }

    #[test]
    fn lista_vazia_continua_vazia() {
        let mut arr: [i32; 0] = [];
        quicksort(&mut arr);
        assert_eq!(arr, []);
    }

    #[test]
    fn lista_com_um_elemento() {
        let mut arr = [42];
        quicksort(&mut arr);
        assert_eq!(arr, [42]);
    }

    #[test]
    fn ordena_negativos() {
        let mut arr = [-5, -2, -10];
        quicksort(&mut arr);
        assert_eq!(arr, [-10, -5, -2]);
    }

    #[test]
    fn ordena_numeros_mistos() {
        let mut arr = [-100, 0, 4, 7, -2, 10];
        quicksort(&mut arr);
        assert_eq!(arr, [-100, -2, 0, 4, 7, 10]);
    }

    #[test]
    fn lista_com_elementos_repetidos() {
        let mut arr = [5, 3, 5, 1, 3, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 3, 3, 5, 5, 5]);
    }

    #[test]
    fn lista_toda_igual() {
        let mut arr = [7, 7, 7, 7];
        quicksort(&mut arr);
        assert_eq!(arr, [7, 7, 7, 7]);
    }

    #[test]
    fn lista_ja_ordenada() {
        // caso classico onde o pivo "ultimo elemento" degrada pra O(n^2):
        // funciona, mas repare no numero de comparacoes se voce instrumentar
        let mut arr = [1, 2, 3, 4, 5];
        quicksort(&mut arr);
        assert_eq!(arr, [1, 2, 3, 4, 5]);
    }

    #[test]
    fn lista_em_ordem_decrescente() {
        let mut arr = [70, 60, 30, 10];
        quicksort(&mut arr);
        assert_eq!(arr, [10, 30, 60, 70]);
    }

    #[test]
    fn funciona_com_chars() {
        let mut arr = ['d', 'a', 'c', 'b'];
        quicksort(&mut arr);
        assert_eq!(arr, ['a', 'b', 'c', 'd']);
    }

    /// Este teste so compila se o bound for `T: Ord` (sem `Copy`).
    /// `String` nao é `Copy` — se voce escrever `T: Ord + Copy`, ele quebra.
    #[test]
    fn funciona_com_tipo_sem_copy() {
        let mut arr = vec![
            String::from("banana"),
            String::from("abacaxi"),
            String::from("caju"),
        ];
        quicksort(&mut arr);
        assert_eq!(arr, vec!["abacaxi", "banana", "caju"]);
    }

    /// Aceita `&mut Vec<T>` por deref coercion, alem de arrays.
    #[test]
    fn aceita_vec_alem_de_array() {
        let mut v = vec![3, 1, 2];
        quicksort(&mut v);
        assert_eq!(v, vec![1, 2, 3]);
    }

    /// Ordena um pedaco do slice sem tocar no resto — prova que a funcao
    /// trabalha sobre o slice emprestado, e nao sobre uma copia.
    #[test]
    fn ordena_apenas_a_fatia_emprestada() {
        let mut arr = [9, 5, 1, 3, 8];
        quicksort(&mut arr[1..4]);
        assert_eq!(arr, [9, 1, 3, 5, 8]);
    }

    /// Lista grande, gerada sem dependencia externa (LCG simples).
    /// Compara com o sort da std pra ter certeza.
    #[test]
    fn lista_grande_bate_com_o_sort_da_std() {
        let mut semente: u64 = 12345;
        let mut arr: Vec<i64> = (0..1000)
            .map(|_| {
                semente = semente.wrapping_mul(6364136223846793005).wrapping_add(1);
                (semente >> 33) as i64 - 1_000_000
            })
            .collect();

        let mut esperado = arr.clone();
        esperado.sort();

        quicksort(&mut arr);
        assert_eq!(arr, esperado);
    }
}
