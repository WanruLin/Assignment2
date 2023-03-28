# CMSC701 Computational Genomics Assignment2
##  Implementing bitvector rank and select, and applying them to a sparse array

### Task1 bit-vector rank

To implement a succinct, constant-time, bit-vector rank operation, I create the struct `RankSupport` and it can implement the following methods:

- `rank1(i)` : Returns the number of 1s in the underlying bit-vector up to position `i` (exclusive).
- `overhead()` : Returns the size of the `RankSupport` data structure (in bits) that is required to support constant-time rank on the current bitvector.
- `save(fname)` : Saves the `RankSupport` data structure for this bit vector to the file `fname`.
- `load(fname)` : Loads the `RankSupport` data structure for this bit vector from the file `fname`.


### Task2 bit-vector select

To implement a succinct, (at most) log-time, bit-vector select operation, I create the struct `SelectSupport` and it can implement the following methods:

- `select1(i)` : Returns the position, in the underlying bit-vector, of the FIRST index, j for which rank1(j) = 1.
- `overhead()` : Returns the size of the `SelectSupport` data structure (in bits) that is required to support log-time select on the current bitvector.
- `save(fname)` : Saves the `SelectSupport` data structure for this bit vector to the file `fname`.
- `load(fname)` : Loads the `SelectSupport` data structure for this bit vector from the file `fname`.

### Task3 sparse array implementation

To implement the sparse array using the bit vector rank and select, I create the `SparseArray` struct and it can implement the following methods:

- `create(size)` : Creates an empty sparse array of length `size` (the size of the underlying bitvector).

- `append(string elem, pos)` : Appends the element `elem` at index `pos` of the sparse array.

- `finalize()` : This will “finalize” the sparse array. At this point, no more elements will be added. This is when I take the opportunity to create `RankSupport` data structures.

- `get_at_rank(r, elem)` : This function places a reference to the r-th present item in the array in the reference `elem`. It returns `true` if there was >= r items in the sparse array and `false` otherwise.

- `get_at_index(r, elem)`: This function looks at the r-th index in the sparse bitvector; if that bit is 1, it fetches the corresponding value and binds it to the reference `elem` and returns `true`, if that bit is a 0, it simply returns `false`.

- `get_index_of(r)` : This function takes as its argument a rank `r` and returns the index in the sparse array where the `r-th` present element appears. **Note** : this is basically just select on the bitvector, except that it returns the position where this element occurs, not one index past that position. If `r` > the weight of the bitvector (i.e. greater than the number of present elements) then this function should return a sentinel value.

- `num_elem_at(r)`: This function returns the count of present elements (1s in the bit vector) up to and including index `r` (**Note**: This is just rank on the bitvector, but it is inclusive rather than exclusive of index `r`).

- `size()` : Returns the size of the sparse array.

- `num_elem()` : Returns the number of present elements in the sparse array (i.e. the number of 1s in the bitvector).

- `save(fname)` : Saves the sparse array to the file `fname`.

- `load(fname)` : Loads the sparse array from the file `fname`.

### `main` function

Within the `main` function, I implement the rank operation, select operation, and sparse array. 

For the rank and select operation(task1 and task2), I randomly generate 100 bit vectors, whose size ranging from 100 to 10000, and I do the rank or select operation 100 times to evaluate the running time of the implementation. And I also evaluate the size of my `RankSupport` and `SelcetSupport` by implementing the `overhead()` method.

For the evaluation of the implementation of sparse array(task3), I generated sparse arrays of a few different lengths (1000, 10000, 100000) and having various sparsity (1%, 5%, 10%). And I implemented `get_at_rank`, `get_at_index`, `get_index_of`, `num_elem_at` these 4 methods for 10 times to evaluate the running time of the implementation. Moreover, I evaluate the size of my `SparseArray` by implementing the `size()` method and compare it to the size when all of the 0 elements were instead explicitly stored as “empty” .
