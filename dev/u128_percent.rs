use rand::Rng;

fn reduce_by_5_percent(value: u128) -> u128 {
    // Calculate the reduction amount without using f64
    let reduction_amount = value * 5 / 100 + 1;

    // Ensure the reduction amount does not exceed the original value
    value.saturating_sub(reduction_amount)
}

#[test]
fn test_reduce_by_5_percent() {
    // Create a random number generator
    let mut rng = rand::thread_rng();

    for _ in 0..10 {
        // Generate a random u128 value
        let original_value: u128 = rng.gen_range(1..1000);

        // Print the original value
        println!("Original value: {}", original_value);

        // Reduce the value by 5%
        let reduced_value = reduce_by_5_percent(original_value);

        // Print the reduced value
        println!("Reduced by 5%: {}\n", reduced_value);
    }
}
