pub struct Instance {
    nodes: Vec<Node>,
    capacity: i32,
    depo_id: i32,   
}

pub struct Node {
    id: i32,
    x: i32,
    y: i32,
    demand: i32,
}

