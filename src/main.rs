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
use core::{Instance, Node};
use plotters::prelude::*;
use plotters::style::full_palette::{BROWN, PINK};
use std::time::Instant;
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
    let instance: Instance = match load_instance("./instances/A-n32-k5.vrp.txt") {
        Ok(ins) => ins,
        Err(e) => {
            eprintln!("Error loading instance: {}", e);
            return;
        }
    };  

    let population_size = 100;
    let generations = 100;
    let iterations = 10000;
    let (ga_distance, ga_path) = time!(genetic_algorithm(&instance, generations, population_size));
    let (gr_distance, gr_path) = time!(greedy_search(&instance));
    let (tabu_distance, tabu_path, tabu_worst, tabu_std) = time!(tabu_search(&instance, iterations, 10));
    let (rs_best, rs_worst, rs_avg, rs_std, rs_path) = time!(random_search(&instance, iterations));
    println!("Tabu Search: best path: {tabu_distance:.1}, worst path: {tabu_worst:.1}, std {tabu_std:.1}");
    println!("Genetic Algorithm: best path: {ga_distance:.1}");
    println!("Greedy search {gr_distance:.1}");
    println!("Random Search: best path: {rs_best:.1}, worst path {rs_worst:.1}, avg {rs_avg:.1}, std {rs_std:.1}");

    plot_best_path(&tabu_path, "tabu_search.png");
    plot_best_path(&ga_path, "ga_search.png");
    plot_best_path(&gr_path, "gr_search.png");
    plot_best_path(&rs_path, "rs_search.png");
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
