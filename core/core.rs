use std::usize;

#[derive(Debug)]
pub struct Instance {
    pub nodes: Vec<Node>,
    pub nodes_id: Vec<i32>,
    pub capacity: i32,
}

impl Instance {
    pub fn new(nodes: Vec<Node>, capacity: i32) -> Self {
        let nodes_id = nodes.iter().map(|node| node.id).collect();

        Self {
            nodes,
            nodes_id,
            capacity,
        }
    }
}
// kazda metoda przytjmuje instancje i evaluator daje
// matrix dystansow i path i32

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub demand: i32,
}

#[derive(Debug)]
pub struct DistanceMatrix {
    matrix: Vec<Vec<f64>>,
    pub demand: Vec<i32>,
}

impl DistanceMatrix {
    pub fn new(nodes: &Vec<Node>) -> Self {
        let size = nodes.len();
        let mut matrix = vec![vec![0.0; size]; size];
        let mut demand = vec![0; size];

        for i in 0..size {
            demand[i] = nodes[i].demand;
            for j in 0..size {
                if i != j {
                    matrix[i][j] = Self::euclidean_distance(&nodes[i], &nodes[j]);
                }
            }
        }

        Self { matrix, demand }
    }

    pub fn euclidean_distance(n1: &Node, n2: &Node) -> f64 {
        ((n2.x as f64 - n1.x as f64).powi(2) + (n2.y as f64 - n1.y as f64).powi(2)).sqrt()
    }

    pub fn get_distance(&self, i: i32, j: i32) -> f64 {
        self.matrix[i as usize][j as usize]
    }
    pub fn get_demand(&self, i: i32) -> i32 {
        self.demand[i as usize]
    }
}
