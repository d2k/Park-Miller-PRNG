use std::time::{Instant}; // Import Instant for timing

fn main() {
    let m: i64 = 2_147_483_647; // 2^31 - 1
    let a: i64 = 16_807; // 7^5

    println!("Enter the starting seed (between 1 and {}):", m - 1);
    let mut input_text = String::new();
    std::io::stdin().read_line(&mut input_text).expect("Failed to read input");
    let mut seed: i64 = input_text.trim().parse().expect("Please enter a valid integer");

    if seed <= 0 || seed >= m {
        println!("The seed must be between 1 and {}", m - 1);
        return;
    }

    let start_seed = seed;
    let mut count: i64 = 0;

    let start_time = Instant::now(); // Start the timer

    loop {
        seed = (a * seed) % m;
        count += 1;

        if seed == start_seed {
            break;
        }
    }

    let duration = start_time.elapsed(); // Calculate the elapsed time

    println!("It took {} iterations for the sequence to return to the start seed.", count);
    println!("Execution Time: {:?}", duration); // Print the execution time
}
