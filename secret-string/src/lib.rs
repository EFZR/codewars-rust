// There is a secret string which is unknown to you.
// Given a collection of random triplets from the string, recover the original string.
// A triplet here is defined as a sequence of three letters such that each letter occurs somewhere before the next in the given string.
// "whi" is a triplet for the string "whatisup".
// As a simplification, you may assume that no letter occurs more than once in the secret string.
// You can assume nothing about the triplets given to you other than that they are valid triplets
// and that they contain sufficient information to deduce the original string. In particular,
// this means that the secret string will never contain letters that do not occur in one of the triplets given to you.

// My solution ->

use std::collections::HashMap;

pub fn recover_secret(triplets: Vec<[char; 3]>) -> String {
    let mut graph: HashMap<char, Vec<char>> = HashMap::new();
    let mut in_degree: HashMap<char, usize> = HashMap::new();

    // Construimos el grafo y calculamos los grados de entrada
    for triplet in &triplets {
        for i in 0..2 {
            graph.entry(triplet[i]).or_default().push(triplet[i + 1]);
            in_degree.entry(triplet[i]).or_insert(0);
            *in_degree.entry(triplet[i + 1]).or_default() += 1;
        }
    }

    // Encontramos el nodo de inicio sin aristas
    let start_node = in_degree
        .iter()
        .find(|&(_, &degree)| degree == 0)
        .map(|(&node, _)| node)
        .unwrap();

    // Ordenacion topologica
    let mut result: Vec<char> = Vec::new();
    let mut queue: Vec<char> = vec![start_node];
    while let Some(current_node) = queue.pop() {
        result.push(current_node);

        if let Some(neighbors) = graph.get(&current_node) {
            for &neighbor in neighbors {
                let entry = in_degree.entry(neighbor).or_default();
                *entry -= 1;
                if *entry == 0 {
                    queue.push(neighbor);
                }
            }
        }
    }

    result.iter().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_test() {
        assert_eq!(
            recover_secret(vec![
                ['t', 'u', 'p'],
                ['w', 'h', 'i'],
                ['t', 's', 'u'],
                ['a', 't', 's'],
                ['h', 'a', 'p'],
                ['t', 'i', 's'],
                ['w', 'h', 's']
            ]),
            "whatisup"
        );
    }
}
