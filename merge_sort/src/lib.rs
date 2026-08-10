/// Implementacao do algoritimo de merge sort
fn merge_sort<T: Copy + Ord>(arr: Vec<T>) -> Vec<T> {
    /* primeiramente deve se dividir a array pela metade consecutivamente ate chegar a unidades
     * singulares len = 1 e.g [3,1,7,5] -> [3] [1] [7] [5]
     * depois deve se comparar os items da duas arrays, juntar em uma array ordenada e.g [3] [1] ->
     * [1, 3] e [7] [5] -> [5, 7], ate todas se juntarem [1,3] [5, 7] -> [1,3,5,7,]
     */
    // top-down recursivo
    //base case
    //
    if arr.len() <= 1 {
        return arr;
    }
    // [1,2,3,4]
    let metade = arr.len() / 2;
    let metade_esquerda = arr[0..metade].to_vec();
    let metade_direita = arr[metade..].to_vec();
    // metade_esquerda [1,2] metade_direita[3,4]
    //vai dividir ate sobrar apenas uma lista de 1 em para esquerda e direita
    //a.[2,1,4,3,] -> metade_esquerda = [2,1] metade_direita = [4,3]
    //a.esquerda = merge_sort([2,1])
    //b. que vai verificar se tem apenas 1 item, como n tem
    //b. ele vai separar entre esquerda e direita denovo
    //b. metade_esquerda = [2] metade_direita = [1]
    //b. esquerda = merge_sort([2]) -> [2] direita = merge_sort([1]) -> [1]
    //b. compara as duas lista, junta em uma ordenada e retorna para o top
    //a. direita faz a mesma coisa de b
    //pega a esquerda e compara cada item com cada item da direita direita,
    //retorna a lista ordenada
    let esquerda = merge_sort(metade_esquerda);
    let direita = merge_sort(metade_direita);
    //checa primeiro item da esquerda com o primeiro da direita
    //caso maior adiciona a lista e aumenta o indice da esquerda em um
    //repete
    //caso o maior for o da direita, adiciona na lista e aumenta o indice da direita em um
    //faca isso ate sobra um so em ma lista e adicionar ele por ultimo
    let mut result = Vec::<T>::new();
    let mut esquerda_idx = 0;
    let mut direita_idx = 0;

    while esquerda_idx < esquerda.len() && direita_idx < direita.len() {
        if esquerda[esquerda_idx] <= direita[direita_idx] {
            result.push(esquerda[esquerda_idx]);
            esquerda_idx += 1;
        } else {
            result.push(direita[direita_idx]);
            direita_idx += 1;
        }
    }
    result.extend_from_slice(&esquerda[esquerda_idx..]);
    result.extend_from_slice(&direita[direita_idx..]);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ordena_corretamente_lista_positiva() {
        assert_eq!(merge_sort(vec![8, 7, 3, 5]), vec![3, 5, 7, 8]);
    }

    #[test]
    fn retorna_lista_vazia_quando_recebe_lista_vazia() {
        assert_eq!(merge_sort(Vec::<i32>::new()), vec![]);
    }

    #[test]
    fn ordena_lista_negativa() {
        assert_eq!(merge_sort([-5, -2, -10].to_vec()), [-10, -5, -2].to_vec());
    }

    #[test]
    fn ordena_numeros_mistos() {
        assert_eq!(
            merge_sort([-100, 0, 4, 7, -2, 10].to_vec()),
            [-100, -2, 0, 4, 7, 10].to_vec()
        );
    }

    #[test]
    fn lista_com_numeros_iguais() {
        assert_eq!(merge_sort([5, 5, 5].to_vec()), [5, 5, 5].to_vec());
    }

    #[test]
    fn lista_com_unico_numero() {
        assert_eq!(merge_sort([10].to_vec()), [10].to_vec());
    }
}
