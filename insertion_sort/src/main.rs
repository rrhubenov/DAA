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

    // TODO: There must be a better way to loop
    for i in 1..sequence.len() {
        let mut j = i as i32;
        while j > 0 && sequence[j as usize] < sequence[(j - 1) as usize] {
            let temp = sequence[j as usize];
            sequence[j as usize] = sequence[(j-1)as usize];
            sequence[(j-1) as usize] = temp;
            j -= 1;
        }
    }

    for num in sequence {
        print!("{} ", num);
    }
}

#[allow(dead_code)]
fn non_increasing_sort(seq: &mut Vec::<i32>) -> &Vec::<i32> {
    for i in 1..seq.len() {
        let key: i32 = seq[i];

        let mut j = (i - 1) as i32;

        while j >= 0 && key > seq[j as usize] {
            seq[(j + 1) as usize] = seq[j as usize];
            j -= 1;
        }
        
        seq[(j + 1) as usize] = key;
    }

    return seq
}