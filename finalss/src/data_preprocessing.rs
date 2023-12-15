/*
This file does a job of data preprocessing that handles 
possible outliers and errors and returns a graph.
 */

use std::collections::HashMap;
use std::error::Error;
use std::fs::File;
use std::io::{self, BufRead};

#[derive(Debug, Clone)]
pub struct Node {
    pub(crate) id: u32,
    surname: String,
    name: String,
    age: u32,
    date: u32,
}

impl Node {
    fn new(id: u32, surname: String, name: String, age: u32, date: u32) -> Node {
     
        Node { id, surname, name, age, date }
    }
}

pub(crate) fn delete_outlier(file_name: String) -> Result<HashMap<u32, Node>, Box<dyn Error>> {

    let file = match File::open(file_name){
        Ok(file) => file,
        Err(error) => panic!("{:?}", error),
    };
   
    let reader = io::BufReader::new(file);
    let mut person_data: HashMap<u32, Node> = HashMap::new();
    let mut id_count = 1;

    for line in reader.lines().skip(1){  
        let line = match line{
            Ok(val) => val,
            Err(error) => panic!("{:?}", error),
        };  
        let temp_vec: Vec<&str> = line.split(",").collect();  

        if let Ok(outlier_value) = temp_vec[2].parse::<i32>() {  
            if outlier_value <= 0 {  
                continue;  
            }
        } else {  
            panic!("Error while reading.");
        }
        let node = Node::new(
            id_count,
            temp_vec[0].to_string(),
            temp_vec[1].to_string(),
            match temp_vec[2].parse(){
                Ok(val) => val,
                Err(error) => panic!("{:?}", error)
            },
            match temp_vec[3].parse(){
                Ok(val) => val,
                Err(error) => panic!("{:?}", error)
            });  

        person_data.insert(id_count, node);  
        id_count += 1; 
    }

    Ok(person_data) 
}

pub(crate) fn create_user_graph(file_name: String, node_data: &Result<HashMap<u32, Node>, Box<dyn Error>>) -> Result<HashMap<u32, Vec<Option<Node>>>, Box<dyn Error>> {

    let file = match File::open(&file_name){
        Ok(file) => file,
        Err(error) => panic!("{:?}", error),
    };
   
    let reader = io::BufReader::new(file);
    let mut graph = HashMap::new();

    for line in reader.lines().skip(1) {
        let line = match line{
            Ok(val) => val,
            Err(error) => panic!("{:?}", error),
        };
       
        let values: Vec<&str> = line.split(",").collect();
        let values: Vec<u32> = vec![
            match values[0].parse(){
                Ok(val) => val,
                Err(error) => panic!("{:?}", error)
            },
            match values[1].parse(){
                Ok(val) => val,
                Err(error) => panic!("{:?}", error)
            },
        ];

        let hashmap_key = values[0];
        let hashmap_value = node_data.as_ref().unwrap().get(&values[1]).cloned();
        graph.entry(hashmap_key).or_insert_with(Vec::new).push(hashmap_value);
    }
    Ok(graph)
}

