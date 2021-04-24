

# Radix sort

## Overview of Radix sort

The meaning of Radix:

The [Radix](https://en.wikipedia.org/wiki/Radix) is actually the
number of unique digits that are possible in a 
[positional numeral system](https://en.wikipedia.org/wiki/Positional_notation).
Basically in what base are we working.
So the gist of Radix sort as you might guess from its name, is to sort numbers(keys)
based on every digit in the keys. This is actually quite close to the way a 
person might sort a list of 10 numbers for example.

Sorting by starting from the left to the right digit is not a great approach.
Try doing it and you'll quickly see why. You'd need to create multiple buckets on each
new digit to remebed the sorting of the previous.

The right (no pun intended) technique is to start from the right most digit and continue left from there.
Since the right most digit is the one with the lowest "weight" on the number.
IF the sorting algorithm we use is a STABLE one (such as Counting sort) the ordering 
of the previous digit keys will remain correct. 

We'll be using Counting sort for Radix.

## Pseudocode

    def RadixSort(A, d: the amount of digits)
        //The right most digit is 1, the left most is r
        for i in 1 to r:
            Sort A by every digit i using CountingSort where k = the radix


## Proof of correctnes

    Proving Radix sort is trivial since we've already prooved the correctness of 
    Counting sort.

## Time complexity

Since we're using CountingSort the time complexity of Radix sort is

T(n) = Theta(d(n + k))

IF k is a constant (for example 10)
T(n) = Theta(dn)

