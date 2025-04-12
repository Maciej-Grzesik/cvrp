use crate::core::{DistanceMatrix, Instance};

pub fn evaluate(distance_matrix: &DistanceMatrix, instance: &Instance, p: &Vec<i32>) -> f64 {
    let mut distance: f64 = 0.0;
    let mut current_capacity: i32 = instance.capacity;
    let mut path = vec![1];
    path.extend(p);

    let mut previous_node: i32 = path[0];
    let depo: i32 = previous_node;
    for &current_node in path.iter().skip(1) {
        let demand = distance_matrix.get_demand(current_node);
        if current_capacity < demand {
            current_capacity = instance.capacity - demand;
            distance += distance_matrix.get_distance(previous_node, depo);
            distance += distance_matrix.get_distance(depo, current_node);
        } else {
            current_capacity -= demand;
            distance += distance_matrix.get_distance(previous_node, current_node);
        }
        previous_node = current_node;
    }
    distance += distance_matrix.get_distance(depo, previous_node);
    distance
}
