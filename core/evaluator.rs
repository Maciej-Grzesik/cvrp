use crate::core::{DistanceMatrix, Instance};

pub fn evaluate(distance_matrix: &DistanceMatrix, instance: &Instance, path: &Vec<i32>) -> f64 {
    let mut distance: f64 = 0.0;
    let mut current_capacity: i32 = instance.capacity;
    println!("current capacity {}", current_capacity);

    let mut previous_node: i32 = path[0] - 1;
    let depo: i32 = previous_node;

    for &current_node in path.iter().skip(1).take(path.len() - 1) {
        let current_node = current_node - 1;
        let current_demand = distance_matrix.get_demand(current_node);
        if current_capacity >= current_demand {
            current_capacity -= current_demand;
            distance += distance_matrix.get_distance(previous_node, current_node);
        } else {
            //println!("Going to depo before node {}", current_node + 1);
            current_capacity = instance.capacity;
            distance += distance_matrix.get_distance(previous_node, depo);
            distance += distance_matrix.get_distance(depo, current_node);
            current_capacity -= current_demand;
        }

        //println!("current node {}", current_node + 1);
        previous_node = current_node;
    }

    distance
}
