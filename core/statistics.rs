use crate::Instance;
use crate::genetic_algorithm::genetic_algorithm;
use crate::random_search::random_search;
use crate::tabu_search::tabu_search;
use statrs::statistics::Statistics;
use std::fs::OpenOptions;
use std::io::{Result, Write};
use std::path::Path;

#[macro_export]
macro_rules! time {
    ($expr:expr) => {{
        let start = std::time::Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        println!("Time elapsed: {:.3}ms", duration.as_secs_f64() * 1000.0);
        result
    }};
}

pub fn run_algorithms<A: Algorithm>(algorithm: A) -> (f64, f64, f64, f64) {
    let mut results = Vec::new();

    for _ in 0..10 {
        let result = algorithm.run();
        results.push(result);
    }
    println!("{}", results.iter().cloned().fold(f64::INFINITY, f64::min));
    (
        results.iter().cloned().fold(f64::INFINITY, f64::min),
        results.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
        results.clone().mean(),
        results.std_dev(),
    )
}

pub trait Algorithm {
    fn run(&self) -> f64;
}

pub struct TabuSearch<'a> {
    pub instance: &'a Instance,
    pub iterations: i32,
    pub tabu_size: usize,
}

impl<'a> Algorithm for TabuSearch<'a> {
    fn run(&self) -> f64 {
        time!(tabu_search(self.instance, self.iterations, self.tabu_size))
    }
}

pub struct GeneticAlgorithm<'a> {
    pub instance: &'a Instance,
    pub generations: i32,
    pub population_size: i32,
    pub mutation_probability: f64,
    pub crossover_probability: f64,
}

impl<'a> Algorithm for GeneticAlgorithm<'a> {
    fn run(&self) -> f64 {
        time!(genetic_algorithm(
            self.instance,
            self.generations,
            self.population_size,
            self.mutation_probability,
            self.crossover_probability,
        ))
    }
}

pub struct RandomSearch<'a> {
    pub instance: &'a Instance,
    pub iterations: i32,
}

impl<'a> Algorithm for RandomSearch<'a> {
    fn run(&self) -> f64 {
        time!(random_search(self.instance, self.iterations))
    }
}

pub fn save_stats_to_file<P>(
    path: P,
    instance_name: &str,
    optimal: f64,
    rs_stats: (f64, f64, f64, f64),
    gr_distance: f64,
    ga_stats: (f64, f64, f64, f64),
    tabu_stats: (f64, f64, f64, f64),
) -> Result<()>
where
    P: AsRef<Path>,
{
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;
    writeln!(
        file,
        "{},{:.0},\
        {:.1},{:.1},{:.1},{:.1},\
        {:.1},\
        {:.1},{:.1},{:.1},{:.1},\
        {:.1},{:.1},{:.1},{:.1}",
        instance_name,
        optimal,
        rs_stats.0, rs_stats.1, rs_stats.2, rs_stats.3,
        gr_distance,
        ga_stats.0, ga_stats.1, ga_stats.2, ga_stats.3,
        tabu_stats.0, tabu_stats.1, tabu_stats.2, tabu_stats.3
    )?;

    Ok(())
}
