use crate::taguchi::{TabuConfig, GaConfig};

pub fn tabu_configs() -> Vec<TabuConfig> {
    vec![
        TabuConfig { tabu_size: 5 },
        TabuConfig { tabu_size: 10 },
        TabuConfig { tabu_size: 15 },
        TabuConfig { tabu_size: 20 },
        TabuConfig { tabu_size: 25 },
        TabuConfig { tabu_size: 30 },
        TabuConfig { tabu_size: 35 },
        TabuConfig { tabu_size: 40 },
        TabuConfig { tabu_size: 45 },
    ]
}

//pub fn ga_configs() -> Vec<GaConfig> {
//    vec![
//        GaConfig { generations: 50,  population_size: 200, crossover_probability: 0.6, mutation_probability: 0.1 },
//        GaConfig { generations: 50,  population_size: 200, crossover_probability: 0.8, mutation_probability: 0.2 },
//        GaConfig { generations: 50,  population_size: 200, crossover_probability: 1.0, mutation_probability: 0.3 },
//        GaConfig { generations: 100, population_size: 100, crossover_probability: 0.8, mutation_probability: 0.3 },
//        GaConfig { generations: 100, population_size: 100, crossover_probability: 1.0, mutation_probability: 0.1 },
//        GaConfig { generations: 100, population_size: 100, crossover_probability: 0.6, mutation_probability: 0.2 },
//        GaConfig { generations: 200, population_size: 50,  crossover_probability: 1.0, mutation_probability: 0.2 },
//        GaConfig { generations: 200, population_size: 50,  crossover_probability: 0.6, mutation_probability: 0.3 },
//        GaConfig { generations: 200, population_size: 50,  crossover_probability: 0.8, mutation_probability: 0.3 },
//        GaConfig { generations: 25,  population_size: 400, crossover_probability: 0.7, mutation_probability: 0.2 },
//    ]
//}

pub fn ga_configs() -> Vec<GaConfig> {
    let gen_levels = [80, 100, 125];
    let pop_levels = [125, 100, 80];
    let cross_levels = [0.7, 0.8, 0.9];
    let mut_levels = [0.1, 0.2, 0.3];

    let mut configs = Vec::new();

    for &generation in &gen_levels {
        for &pop in &pop_levels {
            if generation * pop == 10_000 {
                for &cross in &cross_levels {
                    for &mutation in &mut_levels {
                        configs.push(GaConfig {
                            generations: generation,
                            population_size: pop,
                            crossover_probability: cross,
                            mutation_probability: mutation,
                        });
                    }
                }
            }
        }
    }

    configs
}
