use crate::statistics::{Algorithm, run_algorithms};
use std::fmt::Debug;
use std::fs::OpenOptions;
use std::io::Write;
use crate::{GeneticAlgorithm, TabuSearch, core::Instance};

static mut INSTANCE_PTR: *const Instance = std::ptr::null();

pub fn set_instance(instance: &Instance) {
    unsafe {
        INSTANCE_PTR = instance as *const Instance;
    }
}

fn get_instance() -> &'static Instance {
    unsafe {
        assert!(!INSTANCE_PTR.is_null(), "INSTANCE_PTR not set");
        &*INSTANCE_PTR
    }
}

pub trait ConfigurableAlgorithm {
    type Config: Debug + Clone;
    fn from_config(config: &Self::Config) -> Self;
}

#[derive(Clone, Debug)]
pub struct GaConfig {
    pub generations: i32,
    pub population_size: i32,
    pub crossover_probability: f64,
    pub mutation_probability: f64,
}

#[derive(Clone, Debug)]
pub struct TabuConfig {
    pub tabu_size: usize,
}

impl<'a> ConfigurableAlgorithm for GeneticAlgorithm<'a> {
    type Config = GaConfig;

    fn from_config(config: &Self::Config) -> Self {
        Self {
            instance: get_instance(),
            generations: config.generations,
            population_size: config.population_size,
            crossover_probability: config.crossover_probability,
            mutation_probability: config.mutation_probability,
        }
    }
}

impl<'a> ConfigurableAlgorithm for TabuSearch<'a> {
    type Config = TabuConfig;

    fn from_config(config: &Self::Config) -> Self {
        Self {
            instance: get_instance(),
            iterations: 10_000,
            tabu_size: config.tabu_size,
        }
    }
}

fn write_results_to_csv<T: std::fmt::Debug>(
    path: &str,
    algorithm_name: &str,
    run_number: usize,
    config: &T,
    stats: (f64, f64, f64, f64),
) -> std::io::Result<()> {
    let file_exists = std::path::Path::new(path).exists();
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    if !file_exists {
        writeln!(file, "algorithm,run,config,best,worst,avg,std_dev")?;
    }

    writeln!(
        file,
        "{},{:?},{:?},{:.3},{:.3},{:.3},{:.3}",
        algorithm_name, run_number, config, stats.0, stats.1, stats.2, stats.3
    )?;
    Ok(())
}

pub fn run_taguchi_experiments<T>(configs: Vec<T::Config>, name: &str)
where
    T: Algorithm + ConfigurableAlgorithm,
    T::Config: std::fmt::Debug,
{
    println!("\n=== Running Taguchi Experiment for {name} ===");

    for (i, config) in configs.iter().enumerate() {
        let algo = T::from_config(config);
        println!("\n[Run {}] Config: {:?}", i + 1, config);
        let stats = run_algorithms(algo);
        println!(
            "{name}: Best: {:.1}, Worst: {:.1}, Avg: {:.1}, Std Dev: {:.1}",
            stats.0, stats.1, stats.2, stats.3
        );

        if let Err(e) = write_results_to_csv(&format!("{}_taguchi_results.csv", name), name, i + 1, config, stats) {
            eprintln!("Error writing CSV: {}", e);
        }
    }
}
