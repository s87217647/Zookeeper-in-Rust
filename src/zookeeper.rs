mod node;

use std::collections::HashSet;
use rand::random;
use crate::node:: Node;

pub struct Zookeeper {
    ensemble: Vec<Node>, // shoud be a vector
    taken_id: HashSet<i32>,

}
impl Zookeeper{
    // user interface
    pub fn new () -> Self{
        Self{ensemble: Vec::new(), taken_id: HashSet::new()}
    }
    // Basic tools for development
    pub fn size(self) -> usize {
        return self.ensemble.len();
    }

    pub fn add_node(mut self){
        // let new_id = rand::rngs()
        let mut new_id = 0;

        while self.taken_id.contains(&new_id){
            new_id = rand::random::<i32>();
        }


        let new_node =  Node::new(new_id, &self);
        self.ensemble.push(new_node);

    }

}


impl Zookeeper{
    // user interface
    pub fn zookeep(&self){
        println!("In the jungle the quite jungle, the lion sleeps tonight; Hence, I, the humble zookeeper is gonna tame this jungle");
    }

    pub fn create(&self){
        return;
    }

    pub fn get(&self){
        return;
    }

    pub fn delete(&self){
        return;
    }

}

impl Zookeeper{
    fn election(){
        //prefer the one with highest zxid


        return;
    }

    fn discovery(){
        return;
    }

    fn sync(){
        return;
    }

    fn broadcast(){
        return;
    }




}

