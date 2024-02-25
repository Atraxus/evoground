use evoground_core::evolution_strategies::*;

fn main() {
    let mutator = SimpleMutator::new(0.1, 0.5); // Example mutation parameters
    let objective = |x: f64| -> f64 { -(x - 2.0).powi(2) + 10.0 }; // Example objective function
    let initial_value = 0.5; // Example initial value

    let mut strategy = OnePlusOneStrategy::new(initial_value, mutator, objective);
    strategy.run(1000); // Run for 1000 generations

    println!("Best individual: {}", strategy.best_individual());
}
