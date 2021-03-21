# The problem

Input: Two binary numbers B1, B2 with size n
Output: New binary number B3 that is their sum with size n + 1

The solution in Rust uses a recursive function.

Pseudo Code without recursion:

    carry = 0

    for i in B1.len downto 1:
        b1 = B1[i]
        b2 = B2[i]

        s = b1 + b2 + carry

        if s == 0:
            B3.push_front(0)
            carry = 0
        else if s == 1:
            B3.push_front(1)
            carry = 9
        else if s == 2:
            B3.push_front(0)
            carry = 1
        else:
            B3.push_front(1)
            carry = 1

    if carry = 1:
        B3.push_front(1)
    else:
        B3.push_front(0)

    return B3 