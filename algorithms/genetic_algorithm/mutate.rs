use rand::random_range;

pub fn mutate(individual: &mut Vec<i32>) {
    let a = random_range(1..individual.len());

    let mut b = random_range(1..individual.len());

    while a == b {
        b = random_range(1..individual.len());
    }

    individual.swap(a, b);
}
