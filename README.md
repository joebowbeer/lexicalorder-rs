# lexicalorder-rs
An application of topological sorting. This implementation is inspired by a
[paragraph](https://en.wikipedia.org/wiki/Topological_sorting#Parallel_algorithms)
in Wikipedia:

*A topological ordering can be constructed in O(log2 n) time using a
polynomial number of processors, putting the problem into the complexity
class NC2 (Cook 1985). One method for doing this is to repeatedly square
the adjacency matrix of the given graph, logarithmically many times,
using min-plus matrix multiplication with maximization in place of
minimization. The resulting matrix describes the longest path distances in
the graph. Sorting the vertices by the lengths of their longest incoming
paths produces a topological ordering (Dekel, Nassimi & Sahni 1981).*

```console
$ curl https://raw.githubusercontent.com/powerlanguage/word-lists/master/word-list-7-letters.txt | cargo run --
['\'', 'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z']
```