use crate::core::{DistanceMatrix, Instance};

pub fn greedy_search(instance: &Instance) -> f64 {
    let distance_matrix: DistanceMatrix = DistanceMatrix::new(&instance.nodes);

    let mut remaining_customers = instance.nodes_id.clone();
    remaining_customers.remove(0);

    let mut current_capacity = instance.capacity;
    let mut total_distance = 0.0;

    let depot = instance.nodes_id[0] - 1;
    let mut current_location = depot;

    while !remaining_customers.is_empty() {
        let mut closest_customer: Option<i32> = None;
        let mut min_distance = f64::INFINITY;
        let mut index_to_remove: Option<usize> = None;

        for (index, &customer) in remaining_customers.iter().enumerate() {
            if current_capacity >= distance_matrix.get_demand(customer - 1) {
                let distance = distance_matrix.get_distance(current_location, customer - 1);
                if distance < min_distance {
                    min_distance = distance;
                    closest_customer = Some(customer - 1);
                    index_to_remove = Some(index);
                }
            }
        }

        if let Some(customer) = closest_customer {
            total_distance += min_distance;
            current_location = customer;
            current_capacity -= distance_matrix.get_demand(customer - 1);
            if let Some(idx) = index_to_remove {
                remaining_customers.remove(idx);
            }
        } else {
            total_distance += distance_matrix.get_distance(current_location, depot);
            current_capacity += instance.capacity;
            current_location = depot;
        }
    }

    total_distance += distance_matrix.get_distance(current_location, depot);

    total_distance
}
