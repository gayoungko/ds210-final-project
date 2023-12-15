/*
This file imports all the created files and includes tests for coding.
 */s
mod data_preprocessing;
mod degree_of_kinship;
mod test_function;

use std::error::Error;
use std::io::{self, BufRead};
use std::ops::Index;

fn main() {
    let user_data_hashmap = data_preprocessing::delete_outlier("user_table.csv".to_string());
    test_function::test_get_user_data(1, &user_data_hashmap);

    let graph_hashmap = data_preprocessing::create_user_graph(
        "friends_table.csv".to_string(), &user_data_hashmap).expect("Cannot find file or user information not valid.");
    test_function::test_get_graph(1, &Ok(graph_hashmap.clone()));

    //returns the starting node and number of people with certain degree of seperation from the starting node
    let n = 1;
    let start_node = 1;
    let result1 = degree_of_kinship::get_number_of_people_in_degree(start_node, n, &Ok(graph_hashmap.clone()));
    println!("Starting node : {}, number of people with {} degree of seperation : {}", start_node, n, result1);

    //returns the 2 individual's degree of seperation and its corresponding path
    let person_id1 = 1;
    let person_id2 = 7;
    let result2 = degree_of_kinship::find_relationship_distance_with_path(person_id1, person_id2, &Ok(graph_hashmap.clone()));
    println!("{} and {} 's degree of seperation : {}, path : {:?}", person_id1, person_id2, result2.0, result2.1);
}
