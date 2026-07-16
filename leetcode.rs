//two sum solution
use std::collections::HashMap;
pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    //comeco a iteracao sobre a array
    //pego a diferenca do item com o alvo
    //itero denovo sobre a array pulando os itens ja iterados e comparando os item n oterados a diferenca
    //retorno o indice quando achar

    //for (idx1, v1) in nums.iter().enumerate() {
    //let dif = target - v1;
    //for (idx2, v2) in nums.iter().enumerate().skip(idx1 + 1){
    //if dif == *v2 {
    //return vec![idx1 as i32, idx2 as i32];
    //}
    //}
    //}
    //Vec::new()

    //comeco a iteracao sobre a array
    //tiro a diferenca entre cada item com o target e armazeno em um hashmap com k = diferenca, v = indx do item, e substituo na array o valor pela sua diferenca
    //itero outra vez sobre a array, agr usando o valor do idnx(diferenca) como key para ver se existe algum key na hashmap que bata com ele, caso sim, retorne o indx ataul do for loop + o valor do key da hashmap em vec i32
    //let mut dif_array: Vec<i32> = Vec::new();
    //let mut idx_map: HashMap<i32, i32> = HashMap::new();
    //for (idx, v) in nums.iter().enumerate() {
    //let dif = target - *v;
    //dif_array.push(dif);
    //idx_map.insert(*v, idx as i32);
    //}
    //for (i, d) in dif_array.iter().enumerate() {
    //if let Some(value_idx) = idx_map.get(d) {
    //if i == *value_idx as usize {
    //continue;
    //}
    //return vec![i as i32, *value_idx];
    //}
    //}

    //ainda pode ser mis simplificado
    //vou precisar do hashmap de valor, idx
    //nao preciso da array de diffs
    // no for loop vou tirar a diferenca do target com o o valor
    // tento procurar o idx no map utilizando .get(&diff)
    // caso caso ache o valor, retorne um vec i32 com o idx atual, e o value encontrado do hashmap
    //caso n ache, insert o o valor e idx do item atual na hashmap
    let mut memory: HashMap<i32, i32> = HashMap::new();
    for (idx, v) in nums.iter().enumerate() {
        let diff = target - *v;
        if let Some(k) = memory.get(&diff) {
            return vec![idx as i32, *k];
        }
        memory.insert(*v, idx as i32);
    }
    Vec::new()
}
