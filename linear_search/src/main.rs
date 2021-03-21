use std::io;

fn main() {
    println!("Enter seq of numbers: ");

    let mut seq_input = String::new();

    io::stdin()
    .read_line(&mut seq_input)
    .ok()
    .expect("Read error");

    let seq: Vec::<i32> = seq_input
    .split_whitespace()
    .map(|s| s.parse::<i32>().expect("Parse error"))
    .collect();

    println!("Enter search target: ");

    let mut target_string = String::new();

    io::stdin()
    .read_line(&mut target_string)
    .ok()
    .expect("Read error");

    let target = target_string.trim().parse::<i32>()
    .ok()
    .expect("Parse error");

    let mut current_index = 0;
    let mut found_index = -1;

    while current_index != seq.len() {
        if seq[current_index] == target {
            found_index = current_index as i32;
            break;
        } else {
            current_index += 1;
        }
    }

    println!("Index: {}", found_index);

}
