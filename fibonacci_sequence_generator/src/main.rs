use std::io;

fn generate_fibonacci_sequence(n: u64) -> Vec<u64> {
    let mut sequence: Vec<u64> = Vec::new();
    sequence.push(1);

    let mut prev:u64 = 0;
    for _i in 1..n {
        let new_last = sequence.last().unwrap() + prev;
        prev = *sequence.last().unwrap();
        sequence.push(new_last);
    }
    return sequence;
}

fn main() {
    println!("Fibonacci Sequence Generator");
    println!("=============================");

    let mut input = String::new();
    println!("Note: due to the limitations of the u64 data type, the maximum number of fibonacci numbers that can be generated is 93.\n");
    println!("Enter the number of fibonacci numbers to generate (between 1 - 93): ");
    io::stdin()
        .read_line(&mut input)
        .expect("Enter a number");

    let input: u64 = input.trim().parse().expect("You must enter a number between 1 - 93!");
    if input < 1 {
        print!("Enter a number greater than 0");
        return;
    }

    let fibonacci_sequence = generate_fibonacci_sequence(input);
    println!("Fibonacci Sequence:\n{:?}", fibonacci_sequence);
}