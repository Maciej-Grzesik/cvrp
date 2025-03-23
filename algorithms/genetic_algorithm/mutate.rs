use rand::random_range;
use crate::core::Node;

pub fn mutate (individual: &mut Vec<Node>) {
    let a = random_range(1..individual.len());
    let b = random_range(1..individual.len());

    individual.swap(a, b);

}
