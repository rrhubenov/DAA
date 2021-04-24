# Counting sort


## Overview of the idea

We can prove that algorithms based only on comparisons take omega(nlgn) time at best.
But if we are given a set of constraints over the elements in the array that needs to be sorted
we can abuse these constraints to create a sorting algorithm that takes linear theta(n) time.

Counting sort is a theta(n) time sorting algorithm that relies on the constraint that
the keys that are being sorted are in a certain range.

Eg. If A is an array, for every element 'a' in A, 'a' belongs to (1, 100).

In other words 'a' CAN be a value of 1 at minimum and CAN be a value of 100 at maximum.
(It's important to emphasize that 'a' DOESN'T need to be either 1 or 100. This constraint only gives us boundaries.)

### Important properties of counting sort
 - Counting sort is not an In-Place algorithm. In other words, counting sort uses auxiliary data structures to 
sort the input array A. 
 - Counting sort is STABLE. Meaning it maintains the order of equal elements. If one 5 was before another 5 in the input array A, the same order for the 
  two elements is maintained in the output array after the algorithm finishes its work.


### How counting sort works.

Given an array A and an integer k, where every a in A: a >= 0, a <= k.
We first need to construct an auxiliary array B where |B| = k.

1. Create an array B where |B| = k  and every 'b' in B: b = 0. Create an array C with the size A.len

2. We read trough array A and for every value 'a' we incrment B[a] += 1.
    Thus the index values in B actually hold the number of occurences of the index in A.

This is the crucial part and the hardest to wrap your head around:
3. Going from left to right, starting from 2, we increment every B[i] with B[i-1] 

Now why do we do this step? This is needed because of the way we
actually sort the elements in the next step. So without the context of the fourth step
this seems meaningless. So make sure you understand what the fourth step does and then return to the
argument for why we actually do this.

The reason we do this is to DISPLACE every B[i] with the
space that B[i-1] needs. Let's say that B[1] = 3. This means that the value 1 has been found 3 times in A.
So ideally we'd want to create 'space' in the resulting sorted array of 3 slots where a 1 will be plugged.
Lets say that B[2] = 2. We'd increment the value by 3 because B[1] = 3. When we actually place the element with 
value 2 we'll place the first one on index 5 and place the second one on index 4. These are the exact places where the 2's
need to be puy.

Another important reason for this step is that we need to maintain the stableness of the algorithm.
If the elements that we wanted to sort were just pure integers, so the keys and the elements themselves
are the same thing, we could have just iterated over the B array and for every 'i' we'd append i to the
resulting array B[i] times.
First of all this would make the algorithm unstable but even worse is that we wouldn't be 
moving the actual elements but just sorting the key values. This wouldn't even be a sorting algorithm in that case.

4. Iterate over the A array backwards and for every 'i', C[B[A[i]]] = A[i]. Then we decrement B[i] -= 1

In other words, for every element A[i] we find the position it needs to be put in C by asking B.
REMEMBER, B holds the place for every element 'a' in A based on the value of 'a'.

Important! We need to iterate trough A backwards to maintain the stability of the algorithm.
If we iterate forwards we would reverse the order of the equal elements. Thus it would be 'Anti-Stable' in some way?


## Pseudo code

    def CountingSort(A, k):

    create B with size k, create C with size A.len

    for i in 1 to k:
        B[i] = 0

    for i in 1 to A.len:
        B[A[i]] += 1

    for i in 2 to k:
        B[i] += B[i-1]

    for i in A.len downto 1:
        C[B[A[i]]] = A[i]
        B[A[i]] -= 1

    return C

## Proof of correctnes using invariants

First for loop is trivial thus we won't be proving it.

### Second for loop proof

We want to prove that the effect of the foor loop is that B now contains
the number of occurences of every unique element in A.
Where B[A[i]] will have the desired occurences.

Second for loop ivariant:
    On every iteration of the loop B contains the number of times every index has been seen in A[1..i-1].

Initialization:
    Before the first iteration i is 1 thus A[1..i-1] is empty. Out invariant is trivially correct since B has only zeroes.

Maintanence:
    Let's assume our invariant is correct. When the for loop iteration ends B[A[i]] will have been incremented by 1.
    Since we know B[A[i]] was the correct number of occurrences of A[i] before the incrementation, we can 
    definetely say that our invariant maintains its properties.

Termination:
    The for loop will terminate when i is equal to A.len + 1
    Because of our invariant we know that B contains our desired count of every element in A[1..i-1] = A[1..A.len] which is
    the whole array A.

QED

### Third for loop proof

For the third for loop we need to prove that every element in B has the sum of all the 
previous elements.

Invariant:
    For every element j in the subarray B[1..i-1], B[j] = sum of all B[m] where m < j + itself

Initialization:
    When the algo first reaches the for loop B[2..i-1] = B[1..1]. Since the first element
    has no previous, our invariant is trivially correct.

Maintanence:
    Let's assume our invariant is correct at the start of an iteration.
    This would mean that B[i-1] is the sum of all B[j] where j < i plus itself.
    The only instruction that is executed is that B[i-1] is added to B[i].
    When i gets incremented we can definetely say that B[1..i-1] has the properties
    of our invariant.

Termination:
    The for loop will terminate when i is equal to k+1.
    Since k is the size of B, B[1..k+1 - 1] is the whole array.
    Thus our invariant is correct for the whole array B

### Fourth for loop proof

We want to prove that the last for loop fills C with the correct permutation of A
such that Counting sort maintains its stableness.

Invariant:
    Every element in A[i+1..A.len] is in the correct position in C.
    Not only that but equal elements maintain their relative position as in A.
    In other words, Counting sort is a stable sorting algorithm.

Initialization:
    The first time the for loop is reached i = A.len. 


## Time complexity analysis

    First for loop runs k times.
    Second for loop runs n times. (n = A.len)
    Third for loop runs k-1 times.
    Fourht for loop runs n times.

    Thus the whole running time is T(n) = k + n + k - 1 + n
    T(n) = 2k + 2n - 1
    Since we're only intrested in the asymptotic time complexity
    we can ignore the constants.
    T(n) = Theta(n + k)

    IF k < n, in other words if the set of possible values in A is less than A.len

    T(n) = Theta(n)

    ELSE

    T(n) = Theta(n + k)

## Appendix

### Why we'd want a sorting algorithm to be stable

In most practical cases we're sorting an array of elements based on ONE of their keys. (eg. id, score, first letter of a string, etc.)
Because of this in some cases we'd want to maintain the order of equal elements in the array since it might matter 
for the end user. 


Eg.
A very common case is that we might want to sort an array based on two cases.
Using a naive solution and sorting the array twice would work BUT only in the case where
the second sorting procedure is STABLE.

Let's say we have an array of users and we want to sort them based on 2 criterias.
First we'd want them sorted by some score that they have (maybe it's a video game scoreboard.)
But if two users have the same score we'd want them sorted lexicographically. So if Amanda and Bob have the same score of say 100
Amanda would come before Bob in the scoreboard (if we're looking at the scoreboard top to bottom.)

Using our naive approach we can first sort the array lexicographically. This way we know that Amanda will absolutely 
come before Bob. After that we can sort based on their score and ONLY IF the second sorting algorithm we're using 
is a stable one, we can guarantee that even tough Amanda and Bob have the same score, Amanda will absolutely still 
come before Bob because A < B.
