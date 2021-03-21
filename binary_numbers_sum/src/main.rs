
use std::io;

fn main() {
    println!("Enter two same size binary numbers: ");

    let mut num1_str = String::new();
    let mut num2_str = String::new();

    io::stdin()
    .read_line(&mut num1_str)
    .ok()
    .expect("Error reading from stdin");

    io::stdin()
    .read_line(&mut num2_str)
    .ok()
    .expect("Error reading from stdin");

    // Remove newlines. This most probably is a stupid solution
    num1_str.pop();
    num2_str.pop();

    if num1_str.len() != num2_str.len() {
        println!("Numbers must be same size!");
        return;
    }

    let mut num1 = Vec::<u8>::new();
    let mut num2 = Vec::<u8>::new();

    for bit in num1_str.as_bytes() {
        if *bit == 48 {
            num1.push(0);
        }
        else if *bit == 49 {
            num1.push(1);
        } else {
            println!("Must be 1 or 0");
            return;
        }
    }

    for bit in num2_str.as_bytes() {
        if *bit == 48 {
            num2.push(0);
        }
        else if *bit == 49 {
            num2.push(1);
        } else {
            println!("Must be 1 or 0");
        }
    }

    let mut result = Vec::<u8>::new();

    binary_sum(num1, num2, 0, &mut result);

    while !result.is_empty() {
        print!("{}", result.pop().ok_or(0).expect("Pop error"));
    }

}

// Recursive binary sum
fn binary_sum(mut vec1: Vec::<u8>, mut vec2: Vec::<u8>, mut carry: u8, result: &mut Vec::<u8>) {
    if vec1.len() == 0 && vec2.len() == 0 && carry == 0 {
        return;
    }

    let mut last_bit1 = 0;
    let mut last_bit2 = 0;

    if vec1.len() != 0 && vec2.len() != 0 {
        last_bit1 = vec1.pop().ok_or(0).expect("Error popping");
        last_bit2 = vec2.pop().ok_or(0).expect("Error popping");
    }

    let sum = last_bit1 + last_bit2 + carry;

    if sum == 0 {
        result.push(0);
        carry = 0;
    } else if sum == 1 {
        result.push(1);
        carry = 0;
    } else if sum == 2 {
        result.push(0);
        carry = 1;
    } else if sum == 3 {
        result.push(1);
        carry = 1;
    }

    binary_sum(vec1, vec2, carry, result);
}
