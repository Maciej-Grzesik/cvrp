use crate::core::{Instance, Node};

pub fn evaluate(instance: &Instance, path: Vec<Node>) -> (f64, Vec<Node>) {
    let mut distance: f64 = 0.0;
    let mut current_capacity = instance.capacity;

    let mut previous_node = path.first();
    let depo = previous_node.unwrap().clone();
    let mut updated_path: Vec<Node> = vec![depo.clone()];

    for i in 1..instance.nodes.len() {
        let current_node = path.get(i);

        if let (Some(previous_node), Some(current_node)) = (previous_node, current_node) {
            distance += calculate_distance(current_node, previous_node);

            if current_capacity - current_node.demand >= 0 {
                current_capacity -= current_node.demand;
                updated_path.push(current_node.clone());
            } else {
                updated_path.push(depo.clone());
                updated_path.push(current_node.clone());
                current_capacity += instance.capacity;
                distance += calculate_distance(previous_node, &depo);
                distance += calculate_distance(&depo, current_node);
                current_capacity -= current_node.demand;
            }
        }
        previous_node = current_node;
    }
    updated_path.push(depo);

    (distance, updated_path)
}

pub fn calculate_distance(n1: &Node, n2: &Node) -> f64 {
    ((n2.x as f64 - n1.x as f64).powi(2) + (n2.y as f64 - n1.y as f64).powi(2)).sqrt()
}
