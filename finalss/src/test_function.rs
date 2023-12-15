/*
This is a test file that is used to test 
if user data and graph is successfully made.
 */
use std::collections::HashMap;
use std::error::Error;
use crate::data_preprocessing::Node;

pub(crate) fn test_get_user_data(userid: u32, user_data: &Result<HashMap<u32, Node>, Box<dyn Error>>){
    // check if user information is correclty retrieved from user id
    println!("{:?}", user_data.as_ref().unwrap().get(&userid));
}

pub(crate) fn test_get_graph(userid: u32, node_data: &Result<HashMap<u32, Vec<Option<Node>>>, Box<dyn Error>>){
    // check if graph is correctly created
    println!("{:?}", node_data.as_ref().unwrap().get(&userid));
}