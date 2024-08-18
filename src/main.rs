use std::{
    cmp::min,
    collections::{hash_map::Entry, HashMap},
    io::{stdin, BufRead, BufReader, Error},
    sync::Arc,
    thread,
};

/// Reads an ordered list of words from stdin and prints the determined
/// character sort order
fn main() {
    let words = read_words(stdin().lock()).expect("read words");
    // println!("{:?}", words);
    println!("{:?}", lexical_order(&words).expect("sorted"));
}

// Returns words read from input
fn read_words<R: BufRead>(input: R) -> Result<Vec<String>, Error> {
    let reader = BufReader::new(input);
    reader.lines().collect()
}

/// Receives a list of words that are sorted according to an unknown
/// character precedence and returns their characters in the determined order
fn lexical_order(words: &[String]) -> Result<Vec<char>, String> {
    let (indexed, chars) = index_chars(words);
    let dim = chars.len();
    let mut dist = adjacency(&indexed, dim);
    let mut pow = 1;
    while pow < dim {
        dist = maxplus(dist);
        pow <<= 1;
    }
    Ok(restore_chars(&sorted_indices(&dist)?, &chars))
}

#[test]
fn lexical_order_test() {
    assert_eq!(
        Ok(vec![]),
        lexical_order(&[].map(|s: &str| s.to_string())),
        "(empty)"
    );
    assert_eq!(
        Ok(vec!['a']),
        lexical_order(&["a", "aa"].map(|s: &str| s.to_string())),
        "a"
    );
    assert_eq!(
        Ok(vec!['a', 'b']),
        lexical_order(&["a", "b"].map(|s: &str| s.to_string())),
        "ab"
    );
    // Test cases from:
    // https://www.geeksforgeeks.org/given-sorted-dictionary-find-precedence-characters/
    assert_eq!(
        Ok(vec!['b', 'a', 'c']),
        lexical_order(&["bca", "aaa", "acb"].map(|s| s.to_string())),
        "bac"
    );
    assert_eq!(
        Ok(vec!['b', 'd', 'a', 'c']),
        lexical_order(&["baa", "abcd", "abca", "cab", "cad"].map(|s| s.to_string())),
        "bdac"
    );
}

/// Given a list of words, converts each char to an index into a list of chars.
/// Returns the sequences of indices and the indexed list of chars.
fn index_chars(words: &[String]) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut char_map = HashMap::new();
    let mut indexed = Vec::with_capacity(words.len());
    let mut chars = Vec::new();
    for word in words {
        let word_chars = word.chars().collect::<Vec<_>>();
        let mut indices = Vec::with_capacity(word_chars.len());
        for c in word_chars {
            let index;
            match char_map.entry(c) {
                Entry::Vacant(entry) => {
                    index = chars.len();
                    entry.insert(index);
                    chars.push(c);
                }
                Entry::Occupied(entry) => {
                    index = *entry.get();
                }
            }
            indices.push(index);
        }
        indexed.push(indices);
    }
    (indexed, chars)
}

/// Given a list of indices and an indexed list of chars,
/// returns list of chars
fn restore_chars(indices: &[usize], chars: &[char]) -> Vec<char> {
    indices.iter().map(|index| chars[*index]).collect()
}

/// Returns adjacency matrix given sorted list of sequences containing
/// indices in range [0..dim)
fn adjacency(indexed: &[Vec<usize>], dim: usize) -> Vec<Vec<usize>> {
    // initialize output
    let mut adj = vec![vec![0; dim]; dim];
    for i in 1..indexed.len() {
        // pred precedes succ in lexical order
        let pred = &indexed[i - 1];
        let succ = &indexed[i];
        // find first position where their ints differ
        for k in 0..min(pred.len(), succ.len()) {
            if pred[k] != succ[k] {
                adj[pred[k]][succ[k]] = 1;
                break;
            }
        }
    }
    adj
}

/// Returns max-plus product of the given distance matrix.
/// Spawns threads to process rows in parallel.
fn maxplus(dist: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let dim = dist.len();
    let num_chunks = thread::available_parallelism()
        .expect("available parallelism")
        .get();
    // Each chunk contains dim.div_ceil(num_chunks) rows
    let chunk_size = (dim + num_chunks - 1) / num_chunks;
    // println!("dim {dim}, num_chunks {num_chunks}, chunk_size {chunk_size}");
    let mut handles = Vec::with_capacity(num_chunks);
    let dist = Arc::new(dist);
    for i in (0..dim).step_by(chunk_size) {
        let j = min(i + chunk_size, dim);
        let dist = dist.clone();
        handles.push(thread::spawn(move || {
            let mut chunk = Vec::with_capacity(j - i);
            for k in i..j {
                chunk.push(maxplus_row(&dist, k));
            }
            chunk
        }));
    }
    let mut prod = Vec::with_capacity(dim);
    for handle in handles {
        prod.extend(handle.join().expect("chunk received"));
    }
    prod
}

/// Returns one row of the max-plus product of the given distance matrix
#[allow(clippy::needless_range_loop)]
fn maxplus_row(dist: &[Vec<usize>], index: usize) -> Vec<usize> {
    let dim = dist.len();
    let mut row = vec![0; dim];
    for j in 0..dim {
        if index == j {
            continue;
        }
        let mut max = dist[index][j];
        for k in 0..dim {
            if dist[index][k] == 0 || dist[k][j] == 0 {
                continue;
            }
            let sum = dist[index][k] + dist[k][j];
            if sum > max {
                max = sum
            }
        }
        row[j] = max
    }
    row
}

// Index usize::MAX is a reserved to identity an unranked element
const UNRANKED: usize = usize::MAX;

/// Returns row indices ordered by the longest distance found in each row
fn sorted_indices(dist: &[Vec<usize>]) -> Result<Vec<usize>, String> {
    let dim = dist.len();
    assert!(dim < UNRANKED);
    let mut indices = vec![UNRANKED; dim];
    for (index, row) in dist.iter().enumerate() {
        // find maximum distance in row
        let max = *row.iter().max().unwrap();
        if max >= dim {
            return Err(format!("Fail! Cycle detected at index {index}"));
        }
        let rank = dim - max - 1;
        if indices[rank] != UNRANKED {
            return Err(format!(
                "Fail! Rank {rank} of index {index} already assigned to index {}",
                indices[rank]
            ));
        }
        indices[rank] = index;
    }
    Ok(indices)
}
