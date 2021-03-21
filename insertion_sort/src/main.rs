use std::io;

fn main() {
    println!("Enter sequence of numbers: ");

    let mut input = String::new();

    io::stdin()
    .read_line(&mut input)
    .ok()
    .expect("Read error");

    let mut sequence: Vec::<i32> = input.split_whitespace()
    .map(|s| s.parse().expect("Parse error"))
    .collect();

    for i in 1..sequence.len() {
        let key: i32 = sequence[i];

        let mut j = (i - 1) as i32;

        while j >= 0 && key < sequence[j as usize] {
            sequence[(j + 1) as usize] = sequence[j as usize];
            j -= 1;
        }

        sequence[(j + 1) as usize] = key; 
    }

    for num in sequence {
        print!("{} ", num);
    }
}

