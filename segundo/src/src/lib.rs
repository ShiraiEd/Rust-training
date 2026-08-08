/// Receber array e retornar o segundo maior numero da lista
fn segundo(arr: &[i32]) -> Option<i32> {
    /*[1,0,-3,7]
     * maior atual esta no 1 e o segundo maior nao tem
     * maior atual ainda 1 e o segundo maior 0
     * maior atual 1 e o segundo maior ainda sera o 0
     * maior atual sera o 7 e o segundo maior sera o 1
     * retornar 1
     */
    if arr.is_empty() {
        return None;
    }

    let mut maior_atual = arr[0];
    let mut segundo_maior = None;

    for &numero in &arr[1..] {
        if numero > maior_atual {
            segundo_maior = Some(maior_atual);
            maior_atual = numero;
        } else if numero < maior_atual && Some(numero) > segundo_maior {
            segundo_maior = Some(numero);
        }
    }

    segundo_maior
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn funcao_segundo_retorna_maior_numero_da_array() {
        let array = [1, 2, 3, 4, 5];
        let result = segundo(&array);
        assert_eq!(result, Some(4));
    }
    #[test]
    fn funcao_segundo_retorna_maior_numero_negativo_da_array() {
        let array = [-2, -4, -5];
        let result = segundo(&array);
        assert_eq!(result, Some(-4));
    }
    #[test]
    fn funcao_segundo_array_vazia() {
        let array = [];
        let result = segundo(&array);
        assert_eq!(result, None);
    }
    #[test]
    fn funcao_segundo_retorna_none_para_lista_identica() {
        let array = [7, 7, 7, 7];
        let result = segundo(&array);
        assert_eq!(result, None);
    }
    #[test]
    fn funcao_segundo_retorna_none_para_um_unico_item_na_lista() {
        let array = [7];
        let result = segundo(&array);
        assert_eq!(result, None);
    }
    #[test]
    fn funcao_segundo_lista_decrescente() {
        let array = [70, 60, 30, 10];
        let result = segundo(&array);
        assert_eq!(result, Some(60));
    }
    #[test]
    fn funcao_segundo_apenas_dois_elementos() {
        let array = [2, 1];
        let result = segundo(&array);
        assert_eq!(result, Some(1));
    }
    #[test]
    fn funcao_segundo_random() {
        let array = [2, 1, 0, -3, -5, 0];
        let result = segundo(&array);
        assert_eq!(result, Some(1));
    }
}
