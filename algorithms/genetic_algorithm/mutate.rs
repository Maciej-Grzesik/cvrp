use rand::{random_range, rngs::ThreadRng};

pub fn mutate(individual: &mut Vec<i32>, rng: &mut ThreadRng) {
    let a = random_range(1..individual.len());

    let mut b = random_range(1..individual.len());

    while a == b {
        b = random_range(1..individual.len());
    }

    individual.swap(a, b);
}
