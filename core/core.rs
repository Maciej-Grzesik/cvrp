#[derive(Debug)]
pub struct Instance {
    pub nodes: Vec<Node>,
    pub capacity: i32,
    pub depo_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub id: i32,
    pub x: i32,
    pub y: i32,
    pub demand: i32,
}
