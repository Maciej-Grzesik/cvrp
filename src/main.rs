mod instance_loader;
#[path = "../core/core.rs"]
mod core;
#[path = "../algorithms/random_search.rs"]
mod random_search;
#[path = "../core/evaluator.rs"]
mod evaluator;

use core::{Node, Instance};
use std::time::Instant;
use plotters::prelude::*;
use plotters::style::full_palette::BROWN;
use crate::random_search::random_search;
use crate::instance_loader::load_instance;

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
    let instance: Instance;
    match load_instance("./instances/A-n32-k5.vrp.txt") {   
        Ok(ins) => {
            instance = ins;
        }   
        Err(e) => {
            eprintln!("Error loading instance: {}", e);
            return;
        }
    }
    
    let (shortest_path, best_index, best_path) = time!(random_search(&instance));

    println!("Shortest path: {}, found at index: {}", shortest_path, best_index);
    for node in &best_path {
        print!("{} ", node.id);
    }

    plot_best_path(&best_path);
}


fn plot_best_path(best_path: &Vec<Node>) {
    let root = BitMapBackend::new("path_plot.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Best Path", ("sans-serif", 50))
        .build_cartesian_2d(0..100, 0..100)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let points: Vec<(i32,i32)> = best_path.iter().map(|node| (node.x, node.y)).collect();
    chart.draw_series(
        points.iter().map(|&(x, y)| {
            Circle::new((x,y),5,ShapeStyle {
                color: BLUE.to_rgba(),
                filled: true,
                stroke_width: 1,
            })
        })
    ).unwrap();

    let mut prev_node: Option<&Node> = None;
    let line_colors = [&RED, &MAGENTA, &GREEN, &BLUE, &BROWN, &YELLOW];
    let mut color_idx = 0;
    for node in best_path {
        if let Some(prev) = prev_node {
            chart.draw_series(LineSeries::new(
                    vec![(prev.x, prev.y), (node.x, node.y)],
                    line_colors[color_idx].to_owned(),
            )).unwrap();
        }
        if node.id == 1 {
            color_idx += 1;
        }
        prev_node = Some(node);
        }
    
    root.present().unwrap();
}
