/*
This file allows us to understand the degree of relationship 
between the 2 specific nodes and its path from one to another.
 */

use std::collections::{HashMap, VecDeque};
use std::collections::vec_deque;
use std::error::Error;
use crate::data_preprocessing::Node;

pub(crate) fn get_number_of_people_in_degree(start_node_id: u32, degree_number: u32, node_data: &Result<HashMap<u32, Vec<Option<Node>>>, Box<dyn Error>>) -> u32{
    let mut count = 0;
    let mut visit = Vec::new();
    let mut queue: VecDeque<(u32, u32)> = vec_deque::VecDeque::new();
    queue.push_back((0, start_node_id));

    while !queue.is_empty(){
        let pop_data = match queue.pop_front(){
            None => { panic!("Value is not present.")},
            Some(i) => i,
        };
        if pop_data.0 == degree_number{
            count += 1;
            continue
        }
        visit.push(pop_data.1);

        if let Some(neighbors) = node_data.as_ref().ok().and_then(|data| data.get(&pop_data.1)) {
            for node in neighbors.iter().filter_map(|opt_node| opt_node.as_ref()) {
                if visit.contains(&node.id) == false{
                    queue.push_back((pop_data.0 + 1, node.id))
                }
            }
        }
    }

    count
}

pub(crate) fn find_relationship_distance_with_path(start_node_id: u32, end_node_id: u32, node_data: &Result<HashMap<u32, Vec<Option<Node>>>, Box<dyn Error>>) -> (u32, Vec<u32>) {
    let mut result = 0;
    let mut path = Vec::new();
    path.push(start_node_id);

    let mut visit = Vec::new();
    let mut queue: VecDeque<(u32, Vec<u32>)> = VecDeque::new();
    queue.push_back((0, path.clone()));  

    while !queue.is_empty() {
        let pop_data = match queue.pop_front() {
            Some(data) => data,
            None => panic!("Value is not present."),
        };

        let current_node = *pop_data.1.last().unwrap();  
        if current_node == end_node_id {
            result = pop_data.0;
            path = pop_data.1.clone();  
            break;
        }
        visit.push(current_node);

        if let Some(neighbors) = node_data.as_ref().ok().and_then(|data| data.get(&current_node)) {
            for node in neighbors.iter().filter_map(|opt_node| opt_node.as_ref()) {
                if !visit.contains(&node.id) {
                    let mut new_path = pop_data.1.clone();
                    new_path.push(node.id);
                    queue.push_back((pop_data.0 + 1, new_path));
                }
            }
        }
    }

    (result, path)
}