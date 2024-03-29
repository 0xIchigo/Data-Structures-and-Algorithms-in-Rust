# Data Structures and Algorithms in Rust

### What are Data Structures and Algorithms?

A data structure is a specific type of data organization, management, and storage format that is usually
optimized for accessing data efficiently. Essentially, it is a way of storing data that can be accessed, retrieved, 
and updated as fast as possible.

There are two types of data structures: linear and non-linear. A linear data structure is one in which its elements
are arranged sequentially and each element is attached to its neighbour. Non-linear data structures, however, are not
ordered sequentially and their elements do not have to have any direct relation to their neighbours.

Examples of linear data structures include: arrays, queues, and linked lists. Examples of non-linear data structures 
include: trees, and graphs.

An algorithm refers to a set of finite instructions typically used to solve a problem, perform a computation, or
accomplish a specific task. A recipe is a common example as it consists of a set of specific instructions for
preparing a certain meal. In computer science, algorithms are used as specifications for performing various 
calculations, data processing, and automated reasoning.

Learning data structures and algorithms helps you understand how to solve common problems in an effective manner
as well as how to evaluate the efficiency of an algorithm, ie. its runtime complexity.

### Time Complexity

Time complexity refers to the amount of time it takes to run an algorithm. Big O Notation is used to give a rough 
idea of how long an algorithm will take to run based on its input size and the amount of steps it takes to complete 
the algorithm. We use Big O Notation to describe runtime complexity in terms of worst case scenario; it 
doesn't matter if the program loops ten or a million times if the rate of change is the same. 

I have added comments at the top of most files regarding the algorithm's runtime complexity using Big O Notation. For
the ciphers, I've decided to include security considerations at the top in place of runtime complexity.

## List of Data Structures and Algorithms

### Search Algorithms
* [Linear Search](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/linear_search.rs)
* [Binary Search](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/binary_search.rs)

### Sorting Algorithms
* [Bubble Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/bubble_sort.rs)
* [Circle Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/circle_sort.rs)
* [Cocktail Shaker Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/cocktail_shaker_sort.rs)
* [Counting Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/counting_sort.rs)
* [Gnome Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/gnome_sort.rs)
* [Insertion Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/insertion_sort.rs)
* [Pancake Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/pancake_sort.rs)
* [Quick Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/quick_sort.rs)
* [Selection Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/selection_sort.rs)
* [Shell Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/shell_sort.rs)
* [Sleep Sort](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/sleep_sort.rs)

### Mathematical Algorithms
* [Archimedean Spiral](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/archimedean_spiral.rs)
* [Factors of an Integer](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/factors_of_an_integer.rs)
* [Julia Set](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/julia_set.rs)
* [Mandelbrot Set](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/mandelbrot_set.rs)
* [Pascal's Triangle](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/pascals_triangle.rs)

### Ciphers
* [Atbash Cipher](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/atbash_cipher.rs)
* [Caesar Cipher](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/caesar_cipher.rs)

### Miscellaneous
* [99 Bottles of Beer](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/99_bottles_of_beer.rs)
* [100 Doors](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/100_doors.rs)
* [Count to Two Thousand](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/two_thousand.rs)
* [Password Generator](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/blob/main/src/password_generator.rs)
* [Proof of Work Blockchain](https://github.com/0xIchigo/Data-Structures-and-Algorithms-in-Rust/tree/main/src/pow_blockchain)