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
#[path = "../algorithms/tabu_search.rs"]
mod tabu_search;
#[path = "../algorithms/genetic_algorithm/tournament_select.rs"]
mod tournament_select;

use crate::genetic_algorithm::genetic_algorithm;
use crate::greedy_search::greedy_search;
use crate::instance_loader::load_instance;
use crate::random_search::random_search;
use crate::tabu_search::tabu_search;
use core::{Instance, Node, DistanceMatrix};
use std::{f64, fs};
use evaluator::evaluate;
use plotters::prelude::*;
use plotters::style::full_palette::{BROWN, PINK};
use std::time::Instant;
use statrs::statistics::Statistics;
use std::path::Path;
use std::io::{self, Write};
use std::fs::{File, OpenOptions};

macro_rules! time {
    ($expr:expr) => {{
        let start = Instant::now();
        let result = $expr;
        let duration = start.elapsed();
        println!("Time elapsed: {:.3}ms", duration.as_secs_f64() * 1000.0);
        result
    }};
}

fn main() {
    //let ins: Instance = match load_instance("./instances/A-n32-k5.vrp.txt") {
    //    Ok(i) => i,
    //    Err(e) => return,
    //};
    //
    //let distance_matrix = DistanceMatrix::new(&ins.nodes);
    //let path = vec![0, 21, 31, 19, 17, 13, 7, 26, 12, 1, 16, 30, 27, 24, 29, 18, 8, 9, 22, 15, 10, 25, 5, 20, 14, 28, 11, 4, 23, 3, 2, 5];
    ////let path = vec![18, 8, 9, 22, 15, 29, 10, 25, 5, 20, 28, 11, 4, 23, 2, 3, 6, 21, 31, 19, 17, 13, 7, 27, 24, 14, 26, 16, 30, 12, 1];
    //let new: Vec<i32> = path.iter().map(|&x| x+1).collect();
    //let dis = evaluate(&distance_matrix, &ins, &new);
    //print!("dista {dis}");


    println!("lest go");
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
                if entry_path.extension().map(|ext| ext == "txt").unwrap_or(false){
                    println!("Processing instance: {:?}", entry_path.display());
                    let instance: Instance = match load_instance(entry_path.to_str().unwrap()) {
                        Ok(ins) => ins,
                        Err(e) => {
                            eprintln!("Error loading instance: {}", e);
                            return;
                        }
                    };  

                    let tabu_size = 10;
                    let population_size = 100;
                    let generations = 100;
                    let iterations = 10000;


                    let tabu_search = TabuSearch {
                        instance: &instance,
                        iterations,
                        tabu_size,
                    };

                    let genetic_algorithm = GeneticAlgorithm {
                        instance: &instance,
                        generations,
                        population_size,
                    };

                    let random_search = RandomSearch {
                        instance: &instance, 
                        iterations,
                    };

                    let tabu_stats = run_algorithms(tabu_search);
                    let ga_stats = run_algorithms(genetic_algorithm);
                    let rs_stats = run_algorithms(random_search);
                    let gr_distance = time!(greedy_search(&instance));

                    let _ = save_stats_to_file("results.txt", entry_path.to_str().unwrap(), tabu_stats, ga_stats, rs_stats, gr_distance);
                }
            },
            Err(e) => {
                eprintln!("Error reading file in directory: {e}");
            }
        }
    }
}


fn plot_best_path(best_path: &Vec<Node>, name: &str) {
    let root = BitMapBackend::new(name, (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Best Path", ("sans-serif", 50))
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let points: Vec<(i32, i32)> = best_path.iter().map(|node| (node.x, node.y)).collect();
    chart
        .draw_series(points.iter().map(|&(x, y)| {
            Circle::new(
                (x, y),
                5,
                ShapeStyle {
                    color: BLUE.to_rgba(),
                    filled: true,
                    stroke_width: 1,
                },
            )
        }))
        .unwrap();

    let mut prev_node: Option<&Node> = None;
    let line_colors = [&RED, &MAGENTA, &GREEN, &BLUE, &BROWN, &YELLOW, &PINK];
    let mut color_idx = 0;
    for node in best_path {
        if let Some(prev) = prev_node {
            chart
                .draw_series(LineSeries::new(
                    vec![(prev.x, prev.y), (node.x, node.y)],
                    line_colors[color_idx].to_owned(),
                ))
                .unwrap();
        }
        if node.id == 1 {
            color_idx += 1;
        }
        prev_node = Some(node);
    }

    root.present().unwrap();
}

fn run_algorithms<A: Algorithm>(algorithm: A) -> (f64, f64, f64, f64)
    {
         let mut results = Vec::new();

         for _ in 0..10 {
             let result = algorithm.run();
             results.push(result);
         }

         (
             results.iter().cloned().fold(f64::INFINITY, f64::min),
             results.iter().cloned().fold(f64::NEG_INFINITY, f64::max),
             results.clone().mean(),
             results.std_dev(),
         )
    }

trait Algorithm {
    fn run(&self) -> f64;
    }

struct TabuSearch<'a> {
    instance: &'a Instance,
    iterations: i32,
    tabu_size: usize,
}

impl<'a> Algorithm for TabuSearch<'a> {
    fn run(&self) -> f64 {
        time!(tabu_search(self.instance, self.iterations, self.tabu_size))
    }
}

struct GeneticAlgorithm<'a> {
    instance: &'a Instance,
    generations: i32,
    population_size: i32,
}

impl<'a> Algorithm for GeneticAlgorithm<'a> {
    fn run(&self) -> f64 {
        time!(genetic_algorithm(self.instance, self.generations, self.population_size))
    }
}

struct RandomSearch<'a> {
    instance: &'a Instance,
    iterations: i32,
}

impl<'a> Algorithm for RandomSearch<'a> {
    fn run(&self) -> f64 {
        time!(random_search(self.instance, self.iterations))
    }
}

fn save_stats_to_file<P>(path: P, instance_name: &str, tabu_stats: (f64, f64, f64, f64), ga_stats: (f64, f64, f64, f64), rs_stats: (f64, f64, f64, f64), gr_distance: f64) -> io::Result<()>
where P: AsRef<Path>
{
    let mut file = OpenOptions::new().create(true).append(true).open(path)?;

    writeln!(file, "Instance: {}", instance_name)?;
    writeln!(file, "Random Search: Best: {:.1}, Worst: {:.1}, Avg: {:.1}, Std Dev: {:.1}", rs_stats.0, rs_stats.1, rs_stats.2, rs_stats.3)?;
    writeln!(file, "Greedy Search: Distance: {:.1}", gr_distance)?;
    writeln!(file, "Genetic Algorithm: Best: {:.1}, Worst: {:.1}, Avg: {:.1}, Std Dev: {:.1}", ga_stats.0, ga_stats.1, ga_stats.2, ga_stats.3)?;
    writeln!(file, "Tabu Search: Best: {:.1}, Worst: {:.1}, Avg: {:.1}, Std Dev: {:.1}", tabu_stats.0, tabu_stats.1, tabu_stats.2, tabu_stats.3)?;

    Ok(())
}
