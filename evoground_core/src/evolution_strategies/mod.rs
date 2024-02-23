// Module for evolution strategies
//
// This module contains the implementation of the evolution strategies algorithm.

pub trait Mutate {
    fn mutate(&self, individual: &mut f64);
}

pub struct SimpleMutator {
    mutation_rate: f64,
    mutation_size: f64,
}

impl SimpleMutator {
    pub fn new(mutation_rate: f64, mutation_size: f64) -> SimpleMutator {
        SimpleMutator {
            mutation_rate,
            mutation_size,
        }
    }
}

impl Mutate for SimpleMutator {
    fn mutate(&self, individual: &mut f64) {
        if rand::random::<f64>() < self.mutation_rate {
            *individual += (rand::random::<f64>() * 2.0 - 1.0) * self.mutation_size;
        }
    }
}

pub trait Select {
    fn select(&self, population: &Vec<f64>) -> Vec<f64>;
}

pub struct SimpleSelector {
    selection_size: usize,
    objective: fn(&f64) -> f64,
}

impl SimpleSelector {
    pub fn new(selection_size: usize, objective: fn(&f64) -> f64) -> SimpleSelector {
        SimpleSelector {
            selection_size,
            objective,
        }
    }
}

impl Select for SimpleSelector {
    fn select(&self, population: &Vec<f64>) -> Vec<f64> {
        let mut population = population.clone();

        // Sort the population based on the objective function's output
        population.sort_by(|a, b| {
            let score_a = (self.objective)(a);
            let score_b = (self.objective)(b);

            // For descending order (higher scores first), swap the order of comparison
            score_b
                .partial_cmp(&score_a)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Select the top `selection_size` elements
        population.truncate(self.selection_size);

        population
    }
}

pub struct EvolutionStrategy<T, U> {
    population: Vec<f64>,
    mutator: T,
    selector: U,
}

impl<T: Mutate, U: Select> EvolutionStrategy<T, U> {
    pub fn new(population: Vec<f64>, mutator: T, selector: U) -> EvolutionStrategy<T, U> {
        EvolutionStrategy {
            population,
            mutator,
            selector,
        }
    }

    pub fn run(&mut self, generations: usize) {
        for _ in 0..generations {
            // Clone and mutate offspring from the current population
            let mut offspring = self.population.clone();
            for individual in &mut offspring {
                self.mutator.mutate(individual);
            }

            // Combine the parent and offspring populations
            let mut combined_population = self.population.clone();
            combined_population.extend(offspring);
            // println!("Combined population: {:?}", combined_population);

            // Environment selection: Select the best individuals from the combined population
            self.population = self.selector.select(&combined_population);
        }
    }

    pub fn print_population(&self) {
        println!("Population: {:?}", self.population);
    }

    pub fn best_individual(&self) -> f64 {
        self.population.last().unwrap().clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng; // Ensure you have the `rand` crate in your `Cargo.toml`

    #[test]
    fn test_one_plus_one_es_multiple_runs() {
        let runs = 100; // Number of runs
        let mut total_distance = 0.0;
        let mut worst_distance = 0.0;

        for _ in 0..runs {
            let selector = SimpleSelector::new(1, |x: &f64| -> f64 { -(x - 2.0).powi(2) + 10.0 });
            let mutator = SimpleMutator::new(0.1, 0.5);
            let mut rand = rand::thread_rng();
            // Initialize the population with one parent
            let initial_population = vec![rand.gen_range(0.0..5.0)];

            let mut strategy = EvolutionStrategy::new(initial_population, mutator, selector);

            // Run the evolutionary strategy for a set number of generations
            strategy.run(1000);

            // Ensure the final population contains one individual
            assert_eq!(strategy.population.len(), 1);

            // Add the distance from the optimal value to the total distance
            total_distance += (strategy.best_individual() - 2.0).abs();

            if (strategy.best_individual() - 2.0).abs() > worst_distance {
                worst_distance = (strategy.best_individual() - 2.0).abs();
            }
        }

        // Calculate the average distance from the optimal value
        let average_distance = total_distance / runs as f64;

        println!(
            "Average distance from optimal value: {}, Worst distance: {}",
            average_distance, worst_distance
        );

        // Optionally, assert on the average distance if there's an expected threshold
        assert!(
            average_distance < 0.1,
            "The average distance from the optimal value is too high: {}",
            average_distance
        );
    }
}
