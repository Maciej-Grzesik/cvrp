use crate::core::{DistanceMatrix, Instance};

pub fn evaluate(distance_matrix: &DistanceMatrix, instance: &Instance, path: &Vec<i32>) -> f64 {
    let mut distance: f64 = 0.0;
    let mut current_capacity: i32 = instance.capacity;

    let mut previous_node: i32 = path[0] - 1;
    let depo: i32 = previous_node;
    for &current_node in path.iter().skip(1) {
        let demand = distance_matrix.get_demand(current_node);
        if current_capacity < demand || (distance_matrix.get_distance(current_node, previous_node) > distance_matrix.get_distance(depo, previous_node) && current_capacity < 60) {
            println!(" ");
            current_capacity = instance.capacity - demand;
            distance += distance_matrix.get_distance(previous_node, depo);
            distance += distance_matrix.get_distance(depo, current_node);
        } else {
            current_capacity -= demand;
            distance += distance_matrix.get_distance(previous_node, current_node);
        }
        print!("{} ", current_node);

        previous_node = current_node; 
    }

    distance += distance_matrix.get_distance(depo, previous_node);
    distance
}
