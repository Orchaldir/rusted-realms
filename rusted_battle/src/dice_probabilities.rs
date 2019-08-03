fn main() {
    let n = 6;
    let max_value = n - 1;
    let min_value = -max_value;
    let total_combinations = n * n;
    let mut sum_combinations = 0;
    let mut sum_probability: f32 = 0.0;

    for i in min_value..max_value + 1 {
        let combinations = n - i32::abs(i);
        let probability = combinations as f32 / total_combinations as f32;
        let probability_ge = 1.0 - sum_probability;

        println!(
            "{} | {} | {} | {}",
            i, combinations, probability, probability_ge
        );

        sum_combinations += combinations;
        sum_probability += probability;
    }

    println!("Sum of combinations: {}", sum_combinations);
    println!("Sum of probabilities: {}", sum_probability);
}
