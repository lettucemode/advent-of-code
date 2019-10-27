use regex::Regex;
use std::collections::hash_map::HashMap;

pub fn solve(input: String) -> (String, u32) {
    let mut towers = get_tower_list(&input);
    let mut root = build_node_tree(&mut towers);
    (root.get_name().to_owned(), find_correct_weight(&mut root))
}

fn find_correct_weight(node: &Node) -> u32 {
    let mut weight_counts: HashMap<u32, u32> = HashMap::new();
    for child in node.get_children() {
        let entry = weight_counts
            .entry(child.get_complete_weight())
            .or_insert_with(|| 0);
        *entry += 1;
    }
    if weight_counts.len() == 1 {
        return 0;
    }
    let unbalanced_weight = weight_counts.iter().find(|x| *x.1 == 1u32).unwrap().0;
    let unbalanced_node = node
        .get_children()
        .iter()
        .find(|x| x.get_complete_weight() == *unbalanced_weight)
        .unwrap();
    let mut correct_weight = find_correct_weight(unbalanced_node);
    if correct_weight == 0 {
        let others_weight = *weight_counts.iter().find(|x| *x.1 > 1u32).unwrap().0;
        correct_weight = unbalanced_node.weight
            - (*unbalanced_weight as i32 - others_weight as i32).abs() as u32;
    }
    correct_weight
}

fn get_tower_list(input: &str) -> HashMap<String, Node> {
    let mut towers: HashMap<String, Node> = HashMap::new();
    let n_pattern = Regex::new("^([a-z]+) [(]([0-9]+)[])]").unwrap();
    let c_pattern = Regex::new("([a-z]+)").unwrap();

    for line in input.split_terminator("\n") {
        let captures = n_pattern.captures(line).unwrap();
        let name = captures.get(1).unwrap().as_str();
        let weight: u32 = captures[2].parse().unwrap();
        let tower = towers
            .entry(name.to_string())
            .or_insert_with(|| Node::new(name, weight));
        tower.set_weight(weight);
        if line.contains("->") {
            let child_str = line.split_terminator("->").nth(1).unwrap().trim();
            tower.set_children_str(child_str);
            for child_cap in c_pattern.captures_iter(child_str) {
                let child = towers
                    .entry(child_cap[1].to_owned())
                    .or_insert_with(|| Node::new(&child_cap[1], 0));
                child.set_has_parent(true);
            }
        }
    }
    towers
}

fn build_node_tree(towers: &mut HashMap<String, Node>) -> Node {
    let root_name = towers
        .iter()
        .find(|&x| !(x.1).has_parent)
        .unwrap()
        .0
        .to_owned();
    let mut root = towers.remove(&root_name).unwrap();
    populate(towers, &mut root);
    root
}

fn populate(towers: &mut HashMap<String, Node>, current: &mut Node) {
    let c_pattern = Regex::new("([a-z]+)").unwrap();
    for cap in c_pattern.captures_iter(&current.get_children_str().to_owned()) {
        let child_node = towers.remove(&cap[1]).unwrap();
        current.get_children_mut().push(child_node);
    }
    for child in current.get_children_mut() {
        populate(towers, child);
    }
}

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<Node>,
    children_str: String,
    has_parent: bool,
}

impl Node {
    fn new(name: &str, weight: u32) -> Node {
        Node {
            name: name.to_owned(),
            weight: weight,
            children: vec![],
            children_str: "".to_owned(),
            has_parent: false,
        }
    }

    fn get_name(&self) -> &str {
        &self.name
    }

    fn set_weight(&mut self, w: u32) {
        self.weight = w;
    }

    fn get_children_mut(&mut self) -> &mut Vec<Node> {
        &mut self.children
    }

    fn get_children(&self) -> &Vec<Node> {
        &self.children
    }

    fn get_children_str(&self) -> &str {
        &self.children_str
    }

    fn set_children_str(&mut self, cs: &str) {
        self.children_str = cs.to_owned();
    }
    fn set_has_parent(&mut self, hp: bool) {
        self.has_parent = hp;
    }

    fn get_complete_weight(&self) -> u32 {
        self.children
            .iter()
            .fold(self.weight, |acc, x| acc + x.get_complete_weight())
    }
}
