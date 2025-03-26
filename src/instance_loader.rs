use crate::core::{Instance, Node};
use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn read_file(fp: &str) -> io::Result<impl Iterator<Item = io::Result<String>>> {
    let file = File::open(fp)?;
    let reader = BufReader::new(file);
    Ok(reader.lines())
}

pub fn load_instance(fp: &str) -> Result<Instance, io::Error> {
    let lines = read_file(fp)?;

    let mut nodes = Vec::new();
    let mut demands = Vec::new();
    let mut capacity = 0;
    let mut depo_id = 0;

    let mut section = "";

    for line in lines {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();

                if parts.is_empty() {
                    continue;
                }

                match parts[0] {
                    "CAPACITY" => {
                        if let Some(cap) = parts.get(2).and_then(|s| s.parse().ok()) {
                            capacity = cap;
                        }
                    }
                    "NODE_COORD_SECTION" => section = "NODE",
                    "DEMAND_SECTION" => section = "DEMAND",
                    "DEPOT_SECTION" => section = "DEPOT",
                    "EOF" => break,
                    _ => match section {
                        "NODE" => {
                            if parts.len() == 3 {
                                if let (Some(id), Some(x), Some(y)) = (
                                    parts.first().and_then(|s| s.parse().ok()),
                                    parts.get(1).and_then(|s| s.parse().ok()),
                                    parts.get(2).and_then(|s| s.parse().ok()),
                                ) {
                                    nodes.push(Node {
                                        id,
                                        x,
                                        y,
                                        demand: 0,
                                    });
                                }
                            }
                        }
                        "DEMAND" => {
                            if parts.len() == 2 {
                                if let (Some(id), Some(demand)) = (
                                    parts.first().and_then(|s| s.parse().ok()),
                                    parts.get(1).and_then(|s| s.parse().ok()),
                                ) {
                                    demands.push((id, demand));
                                }
                            }
                        }
                        "DEPOT" => {
                            if let Some(id) = parts.first().and_then(|s| s.parse::<i32>().ok()) {
                                if id == -1 {
                                    break;
                                }
                                //depo_id = id;
                            }
                        }
                        _ => {}
                    },
                }
            }
            Err(e) => return Err(e),
        }
    }

    for (id, demand) in demands {
        if let Some(node) = nodes.iter_mut().find(|n| n.id == id) {
            node.demand = demand;
        }
    }

    let instance = Instance::new(nodes, capacity);
    Ok(instance)
}
