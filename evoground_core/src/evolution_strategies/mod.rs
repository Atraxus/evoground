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

pub struct OnePlusOneStrategy {
    individual: f64,
    mutator: SimpleMutator,
    objective: fn(f64) -> f64,
}

impl OnePlusOneStrategy {
    pub fn new(initial_value: f64, mutator: SimpleMutator, objective: fn(f64) -> f64) -> Self {
        OnePlusOneStrategy {
            individual: initial_value,
            mutator,
            objective,
        }
    }

    pub fn run(&mut self, generations: usize) {
        for _ in 0..generations {
            let mut offspring = self.individual;
            self.mutator.mutate(&mut offspring);
            if (self.objective)(offspring) > (self.objective)(self.individual) {
                self.individual = offspring;
            }
        }
    }

    pub fn best_individual(&self) -> f64 {
        self.individual
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng; // Import necessary items from the outer module

    #[test]
    fn test_one_plus_one_es_multiple_runs() {
        let runs = 100; // Number of runs
        let mut total_distance = 0.0;
        let mut worst_distance = 0.0;

        for _ in 0..runs {
            let mutator = SimpleMutator::new(0.1, 0.5); // Example mutation parameters
            let objective = |x: f64| -> f64 { -(x - 2.0).powi(2) + 10.0 }; // Example objective function
            let mut rand = rand::thread_rng();
            let initial_value = rand.gen_range(0.0..5.0); // Initialize the individual with a random value

            let mut strategy = OnePlusOneStrategy::new(initial_value, mutator, objective);

            // Run the evolutionary strategy for 1000 generations
            strategy.run(1000);

            let distance = (strategy.best_individual() - 2.0).abs();
            total_distance += distance;

            // Update worst distance if the current run's distance is greater
            if distance > worst_distance {
                worst_distance = distance;
            }
        }

        // Calculate the average distance from the optimal value
        let average_distance = total_distance / runs as f64;

        println!(
            "Average distance from optimal value: {}, Worst distance: {}",
            average_distance, worst_distance
        );

        // Assert on the average distance to check the effectiveness of the strategy
        assert!(
            average_distance < 0.1,
            "The average distance from the optimal value is too high: {}",
            average_distance
        );
    }
}
