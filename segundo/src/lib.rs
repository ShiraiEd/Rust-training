/// Receber array e retornar o segundo maior item da lista
fn segundo<T: Ord + Copy>(arr: &[T]) -> Option<T> {
    /*[1,0,-3,7]
     * maior atual esta no 1 e o segundo maior nao tem
     * maior atual ainda 1 e o segundo maior 0
     * maior atual 1 e o segundo maior ainda sera o 0
     * maior atual sera o 7 e o segundo maior sera o 1
     * retornar 1
     */
    // if arr.is_empty() {
    //     return None;
    // }
    //
    // let mut maior_atual = arr[0];

    //usar o metodo split_first para checar se ha elementos e pegar o primeiro e o resto
    let (&primeiro, resto) = arr.split_first()?;

    let mut maior_atual = primeiro;
    let mut segundo_maior = None;

    for &numero in resto
    /* antes &arr[1..] em vez de resto */
    {
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
        assert_eq!(segundo(&[1, 2, 3, 4, 5]), Some(4));
    }
    #[test]
    fn funcao_segundo_retorna_maior_numero_negativo_da_array() {
        assert_eq!(segundo(&[-2, -4, -5]), Some(-4));
    }
    #[test]
    fn funcao_segundo_array_vazia() {
        assert_eq!(segundo::<i32>(&[]), None);
    }
    #[test]
    fn funcao_segundo_retorna_none_para_lista_identica() {
        assert_eq!(segundo(&[7, 7, 7, 7]), None);
    }
    #[test]
    fn funcao_segundo_retorna_none_para_um_unico_item_na_lista() {
        assert_eq!(segundo(&[7]), None);
    }
    #[test]
    fn funcao_segundo_lista_decrescente() {
        let array = [70, 60, 30, 10];
        let result = segundo(&array);
        assert_eq!(result, Some(60));
    }
    #[test]
    fn funcao_segundo_apenas_dois_elementos() {
        assert_eq!(segundo(&[2, 1]), Some(1));
    }
    #[test]
    fn funcao_segundo_random() {
        assert_eq!(segundo(&[2, 1, 0, -3, -5, 0]), Some(1));
    }
    #[test]
    fn segundo_funciona_com_chars() {
        assert_eq!(segundo(&['a', 'b', 'c']), Some('b'));
    }

    #[test]
    fn segundo_funciona_com_bool() {
        assert_eq!(segundo(&[true, false]), Some(false));
    }
}
