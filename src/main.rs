#[path = "../config/algorithms_config.rs"]
mod algorithms_config;
#[path = "../core/core.rs"]
mod core;
#[path = "../algorithms/genetic_algorithm/crossover.rs"]
mod crossover;
#[path = "../core/evaluator.rs"]
mod evaluator;
#[path = "../algorithms/genetic_algorithm/genetic_algorithm.rs"]
mod genetic_algorithm;
#[path = "../algorithms/greedy_search.rs"]
mod greedy_search;
mod instance_loader;
#[path = "../algorithms/genetic_algorithm/mutate.rs"]
mod mutate;
#[path = "../algorithms/random_search.rs"]
mod random_search;
#[path = "../algorithms/genetic_algorithm/roulette_select.rs"]
mod roulette_select;
#[path = "../core/statistics.rs"]
mod statistics;
#[path = "../algorithms/tabu_search.rs"]
mod tabu_search;
#[path = "../core/taguchi.rs"]
mod taguchi;
#[path = "../algorithms/genetic_algorithm/tournament_select.rs"]
mod tournament_select;

use algorithms_config::{ga_configs, tabu_configs};
use taguchi::{run_taguchi_experiments, set_instance};

use crate::core::Instance;
use crate::greedy_search::greedy_search;
use crate::instance_loader::load_instance;
use crate::statistics::{
    GeneticAlgorithm, RandomSearch, TabuSearch, run_algorithms, save_stats_to_file,
};
use std::fs;

fn main() {
    let paths = match fs::read_dir("./instances") {
        Ok(paths) => paths,
        Err(e) => {
            println!("Error reading directory: {e}");
            return;
        }
    };

    for entry in paths {
        match entry {
            Ok(entry) => {
                let entry_path = entry.path();
                if entry_path
                    .extension()
                    .map(|ext| ext == "txt")
                    .unwrap_or(false)
                {
                    println!("Processing instance: {:?}", entry_path.display());
                    let instance: Instance = match load_instance(entry_path.to_str().unwrap()) {
                        Ok(ins) => ins,
                        Err(e) => {
                            eprintln!("Error loading instance: {}", e);
                            return;
                        }
                    };
                    //set_instance(&instance);
                    //run_taguchi_experiments::<TabuSearch>(tabu_configs(), "Tabu");
                    //run_taguchi_experiments::<GeneticAlgorithm>(ga_configs(), "Ga");
                    //break;

                    let tabu_size = 15;
                    let population_size = 80;
                    let generations = 125;
                    let iterations = 10000;
                    let mutation_probability = 0.3;
                    let crossover_probability = 0.9;
                    //
                    //let tabu_search = TabuSearch {
                    //    instance: &instance,
                    //    iterations,
                    //    tabu_size,
                    //};
                    //
                    let genetic_algorithm = GeneticAlgorithm {
                        instance: &instance,
                        generations,
                        population_size,
                        mutation_probability,
                        crossover_probability,
                    };
                    //
                    //let random_search = RandomSearch {
                    //    instance: &instance,
                    //    iterations,
                    //};
                    //
                    //let tabu_stats = run_algorithms(tabu_search);
                    let ga_stats = run_algorithms(genetic_algorithm);
                    //let rs_stats = run_algorithms(random_search);
                    //let gr_distance = time!(greedy_search(&instance));
                    //
                    let tabu_stats = (0.0, 0.0, 0.0, 0.0);
                    let rs_stats = (0.0, 0.0, 0.0, 0.0);
                    let gr_distance = 0.0;
                    let _ = save_stats_to_file(
                        "ga_results.csv",
                        entry_path.to_str().unwrap(),
                        0.0,
                        rs_stats,
                        gr_distance,
                        ga_stats,
                        tabu_stats,
                    );
                }
            }
            Err(e) => {
                eprintln!("Error reading file in directory: {e}");
            }
        }
    }
}
