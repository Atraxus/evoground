use evoground_core::evolution_strategies::*;

fn main() {
    // Example objective function
    let objective_fn = |x: &f64| -> f64 { -(x - 2.0) * (x - 2.0) + 10.0 };

    // Initialize the population with random values or based on some criteria
    let initial_population = vec![0.5, 1.5, 2.5, 3.5, 4.5];

    // Instantiate the mutator and selector with desired parameters
    let mutator = SimpleMutator::new(0.1, 0.5); // Example parameters
    let selector = SimpleSelector::new(3, objective_fn); // Selecting top 3 individuals

    // Create an EvolutionStrategy instance
    let mut strategy = EvolutionStrategy::new(initial_population, mutator, selector);

    // Run the evolutionary strategy for a specified number of generations
    strategy.run(100); // Example: 100 generations

    // Optionally, print the final population to observe the outcome
    strategy.print_population();
}
