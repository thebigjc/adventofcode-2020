static INPUT: &str = include_str!("../day7.txt");
use regex::Regex;
use petgraph::graph::{DiGraph, NodeIndex};
use petgraph::{Incoming, Outgoing};
//use petgraph::dot::{Dot, Config};
use std::collections::HashMap;
use std::collections::HashSet;
use petgraph::visit::EdgeRef;

fn all_outer_bags(dig : &DiGraph<&str, usize>, x: NodeIndex) -> HashSet<NodeIndex> {
    let ns = dig.neighbors_directed(x, Incoming);
    let mut set = HashSet::new();

    for n in ns {
        set.insert(n);
        set = &set | &all_outer_bags(&dig, n);
    }

    return set;
}

fn count_inner_bags(dig : &DiGraph<&str, usize>, x: NodeIndex) -> usize {
    let es = dig.edges_directed(x, Outgoing);
    let mut cnt = 0;

    for e in es {
        println!("{}, {}", dig.node_weight(e.target()).unwrap(), e.weight());

        let inner_count = count_inner_bags(&dig, e.target());
        println!("inner {} - {}", dig.node_weight(e.target()).unwrap(), inner_count);

        cnt += e.weight() + e.weight() * inner_count;
    }

    return cnt;
}

fn main() {
    let re1 = Regex::new(r"^(.+) bags contain (.*)$").unwrap();
    let re2 = Regex::new(r"(\d+) ([^0-9]*) bags?[,\.]|no other bags\.").unwrap();

    let mut dig = DiGraph::<&str, usize>::new(); 
    let mut node_set = HashMap::new();

    for l in INPUT.lines() {
        let cap = re1.captures(l).unwrap();

        let node_name = cap.get(1).map_or("", |m| m.as_str());

        let new_node = dig.add_node(node_name);
        node_set.insert(node_name, new_node);
    }

    for l in INPUT.lines() {
        let cap = re1.captures(l).unwrap();
        let node_name = cap.get(1).map_or("", |m| m.as_str());
        let inner_bags = cap.get(2).map_or("", |m| m.as_str());

        if inner_bags == "no other bags." {
            continue;
        }

        let outer_bag = node_set.get(node_name).unwrap();

        for caps in re2.captures_iter(inner_bags) {
            let inner_bag_name = caps.get(2).unwrap().as_str();
            let inner_bag = node_set.get(inner_bag_name).unwrap();
            let cnt : usize = caps.get(1).unwrap().as_str().parse().unwrap();

            dig.add_edge(*outer_bag, *inner_bag, cnt);
        }
    }

    let my_bag = node_set.get("shiny gold").unwrap();
    let bag_set = all_outer_bags(&dig, *my_bag);
    println!("{:?}", bag_set.len());

    let bag_cnt = count_inner_bags(&dig, *my_bag);
    println!("{:?}", bag_cnt);
}
