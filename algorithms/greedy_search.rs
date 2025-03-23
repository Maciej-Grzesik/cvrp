use crate::core::{Instance, Node};
use crate::evaluator::calculate_distance;

pub fn greedy_search(instance: &Instance) -> (f64, Vec<Node>) {
    let mut path: Vec<Node> = Vec::new();
    let mut remaining_customers = instance.nodes.clone();
    let mut current_capacity = instance.capacity;
    let mut total_distance = 0.0;

    let mut current_location = instance.nodes[0].clone();

    path.push(current_location.clone());

    while !remaining_customers.is_empty() {
        let mut closest_customer: Option<&Node> = None;
        let mut min_distance = f64::INFINITY;
        let mut index_to_remove = 0;

        for (i, customer) in remaining_customers.iter().enumerate() {
            if current_capacity >= customer.demand {
                let distance = calculate_distance(&current_location, customer);
                if distance < min_distance {
                    min_distance = distance;
                    closest_customer = Some(customer);
                    index_to_remove = i;
                }
            }
        }

        if let Some(customer) = closest_customer {
            path.push(customer.clone());
            total_distance += min_distance;
            current_location = customer.clone();
            current_capacity -= customer.demand;
            remaining_customers.remove(index_to_remove);
        } else {
            total_distance += calculate_distance(&current_location, &instance.nodes[0]);
            path.push(instance.nodes[0].clone());
            current_capacity += instance.capacity;
            current_location = instance.nodes[0].clone();
        }
    }

    total_distance += calculate_distance(&current_location, &instance.nodes[0]);
    path.push(instance.nodes[0].clone());


    (total_distance, path)
}


