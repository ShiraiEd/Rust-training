//use std::collections::HashMap;

fn main() {
    let mut a = ListNode::new(1);
    let mut b = ListNode::new(2);
    let c = ListNode::new(3);
    b.next = Some(Box::new(c));
    a.next = Some(Box::new(b));

    add_two_numbers(Some(Box::new(a)), None);
}

//add two numbers
// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub fn add_two_numbers(
    l1: Option<Box<ListNode>>,
    l2: Option<Box<ListNode>>,
) -> Option<Box<ListNode>> {
    let mut n1: Vec<i32> = Vec::new();
    let mut n2: Vec<i32> = Vec::new();

    let mut cursor = l1;

    while let Some(node) = &cursor {
        n1.push(node.val);
        cursor = cursor.unwrap().next;
    }

    cursor = l2;

    while let Some(node) = &cursor {
        n2.push(node.val);
        cursor = cursor.unwrap().next;
    }

    println!("{:?}", n1);
    println!("{:?}", n2);

    let mut head: Option<Box<ListNode>> = None;
    let mut carry = 0;
    let mut r: Vec<i32> = Vec::new();
    let mut count = 0;

    println!("{:?}", n1);
    println!("{:?}", n2);

    if n1.len() > n2.len() {
        for (i, v) in n1.iter().enumerate() {
            if i >= n2.len() {
                count = *v + carry;
            } else {
                count = *v + n2[i] + carry;
            }
            r.push(count % 10);
            carry = count / 10;
        }
    } else if n2.len() > n1.len() {
        for (i, v) in n2.iter().enumerate() {
            if i >= n1.len() {
                count = *v + carry;
            } else {
                count = *v + n1[i] + carry;
            }
            r.push(count % 10);
            carry = count / 10;
        }
    } else {
        for (i, v) in n1.iter().enumerate() {
            count = *v + n2[i] + carry;
            r.push(count % 10);
            carry = count / 10;
        }
    }

    if carry > 0 {
        r.push(carry);
    }

    let r: Vec<&i32> = r.iter().rev().collect();

    println!("{:?}", r);

    for e in r {
        head = Some(Box::new(ListNode {
            val: *e,
            next: head,
        }));
    }
    println!("{:?}", head);
    head
}

//two sum solution
// pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
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
// let mut memory: HashMap<i32, i32> = HashMap::new();
// for (idx, v) in nums.iter().enumerate() {
//     let diff = target - *v;
//     if let Some(k) = memory.get(&diff) {
//         return vec![idx as i32, *k];
//     }
//     memory.insert(*v, idx as i32);
// }
// Vec::new()
// }
