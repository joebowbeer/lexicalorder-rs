use std::{
    collections::{hash_map::Entry, HashMap},
    io::{stdin, BufRead, BufReader},
};

/// Reads an ordered list of words from stdin and prints the determined
/// character sort order
fn main() {
    let words = read_words(stdin().lock());
    println!("{:?}", words);
    let (slices, chars) = index_chars(words);
    for slice in slices {
        println!("{:?}", String::from_iter(restore_chars(slice, &chars)));
    }
}

// Returns words read from input
fn read_words<R: BufRead>(input: R) -> Vec<String> {
    let reader = BufReader::new(input);
    reader.lines().map(|s| s.expect("input lines")).collect()
}

/// Receives a list of words that are sorted according to an unknown
/// character precedence and returns their characters in the determined order
fn _lexical_order(words: Vec<String>) -> Vec<char> {
    let (slices, chars) = index_chars(words);
    let dim = chars.len();
    let mut dist = _adjacency(slices, dim);
    let mut pow = 1;
    while pow < dim {
        dist = _maxplus(dist);
        pow <<= 1;
    }
    restore_chars(_sorted_indices(dist), &chars)
}

/// Given a list of words, converts each char to an index into a list of chars.
/// Returns the indices and the indexed list of chars.
fn index_chars(words: Vec<String>) -> (Vec<Vec<usize>>, Vec<char>) {
    let mut char_map = HashMap::new();
    let mut slices = Vec::with_capacity(words.len());
    let mut chars = Vec::new();
    for word in words {
        let mut indices = Vec::new();
        for c in word.chars() {
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
        slices.push(indices);
    }
    (slices, chars)
}

/// Given a list of indices and an indexed list of chars,
/// returns list of chars
fn restore_chars(indices: Vec<usize>, chars: &[char]) -> Vec<char> {
    indices.into_iter().map(|index| chars[index]).collect()
}

/// Returns adjacency matrix given sorted list of slices containing
/// indices in range [0..dim)
fn _adjacency(_input: Vec<Vec<usize>>, _dim: usize) -> Vec<Vec<usize>> {
    // // initialize output
    // let output = make([][]int, dim)
    // for i := range out {
    // 	out[i] = make([]int, dim)
    // }
    // for i := 1; i < len(in); i++ {
    // 	// pred precedes succ in lexical order
    // 	pred := in[i-1]
    // 	succ := in[i]
    // 	// find first position where their ints differ
    // 	for k := 0; k < len(pred) && k < len(succ); k++ {
    // 		if pred[k] != succ[k] {
    // 			out[pred[k]][succ[k]] = 1
    // 			break
    // 		}
    // 	}
    // }
    Vec::new()
}

/// Returns max-plus product of the given distance matrix
fn _maxplus(_input: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    // dim := len(in)
    // out := make([][]int, dim)
    // var wg sync.WaitGroup
    // chunkSize := (dim + numChunks - 1) / numChunks
    // for i := 0; i < dim; i += chunkSize {
    // 	end := i + chunkSize
    // 	if end > dim {
    // 		end = dim
    // 	}
    // 	wg.Add(1)
    // 	go func(m, n int) {
    // 		defer wg.Done()
    // 		for i := m; i < n; i++ {
    // 			out[i] = maxplusRow(in, i)
    // 		}
    // 	}(i, end)
    // }
    // wg.Wait()
    Vec::new()
}

/// Returns one row of the max-plus product of the given distance matrix
fn _maxplus_row(_input: Vec<Vec<usize>>, _i: usize) -> Vec<usize> {
    // dim := len(in)
    // row := make([]int, dim)
    // for j := 0; j < dim; j++ {
    // 	if i == j {
    // 		continue
    // 	}
    // 	max := in[i][j]
    // 	for k := 0; k < dim; k++ {
    // 		if in[i][k] == 0 || in[k][j] == 0 {
    // 			continue
    // 		}
    // 		if sum := in[i][k] + in[k][j]; sum > max {
    // 			max = sum
    // 		}
    // 	}
    // 	row[j] = max
    // }
    Vec::new()
}

/// Returns row indices ordered by the longest distance found in each row
fn _sorted_indices(_dist: Vec<Vec<usize>>) -> Vec<usize> {
    // dim := len(dist)
    // out := make([]int, dim)
    // for i := range out {
    // 	out[i] = -1
    // }
    // for index := range dist {
    // 	// find maximum distance in row
    // 	max := 0
    // 	for _, d := range dist[index] {
    // 		if d > max {
    // 			max = d
    // 		}
    // 	}
    // 	if max >= dim {
    // 		log.Fatalf("Fail! Cycle detected at index %d", index)
    // 	}
    // 	rank := dim - max - 1
    // 	if out[rank] != -1 {
    // 		log.Fatalf(
    // 			"Fail! Rank %d of index %d already assigned to index %d",
    // 			rank, index, out[rank])
    // 	}
    // 	out[rank] = index
    // }
    Vec::new()
}
