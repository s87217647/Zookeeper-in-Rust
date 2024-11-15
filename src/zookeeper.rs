
use std::cell::RefCell;
use std::collections::{HashMap, HashSet};
use std::rc::{Rc, Weak};
use rand::random;
use crate::node::{Node};



pub struct Zookeeper{
    pub ensemble: Vec<Node>, // vector or map
    taken_id: HashSet<i32>,
    leader_id: Option<i32>
}
impl Zookeeper{
    // user interface
    pub fn new () -> Rc<RefCell<Zookeeper>>{
        let zk = Self{ensemble: Vec::new(), taken_id: HashSet::new(), leader_id: None};
        return Rc::new(RefCell::new(zk))
    }
    // Basic tools for development
    pub fn size(zk: Rc<RefCell<Zookeeper>>) -> usize {
        return zk.borrow_mut().ensemble.len();
    }

    pub fn add_node(zk: Rc<RefCell<Zookeeper>>){
        let mut new_id = random::<i32>();

        while zk.borrow().taken_id.contains(&new_id){
            new_id = rand::random::<i32>();
        }
        zk.borrow_mut().taken_id.insert(new_id);

        let new_node:Node = Node::new (new_id, zk.clone());

        zk.borrow_mut().ensemble.push(new_node);

        // zk.borrow_mut().ensemble.push(new_node);
        // drop(zk);
        // drop(zk);
        // println!("{}", Rc::strong_count(&zk))
    }
}
//
//
// impl Zookeeper<'_>{
//     // user interface
//     pub fn zookeep(&self){
//         println!("In the jungle the quite jungle, the lion sleeps tonight; Hence, I, the humble zookeeper is gonna tame this jungle");
//     }
//
//     pub fn create(&self){
//         return;
//     }
//
//     pub fn get(&self){
//         return;
//     }
//
//     pub fn delete(&self){
//         return;
//     }
//
// }
//
impl Zookeeper{
    pub fn board_cast(zk: Rc<RefCell<Zookeeper>>, msg: Message, sender_id: i32){
        // broadcast the massage to all but the sender
        for n in zk.borrow().ensemble.iter(){
            if(n.id != sender_id){
                Node::receive_msg(n, msg.clone());
            }
        }
    }

    pub fn run(){
        // it should be just running, right? keep on running
    }

    pub fn broadcast_phase(zk: Rc<RefCell<Zookeeper>>){



    }

    pub fn election(zk: Rc<RefCell<Zookeeper>>){
        // Self:: broadcast(Message{msg_type: String::from("looking"), })

        // //prefer the one with highest zxid
        // let mut vote_occurrences = HashMap::new();
        //
        // for n in zk.borrow().ensemble.iter(){
        //     let msg = Message{msg_type:"looking".to_string(), content: "empty".to_string()};
        //     let v = Node:: receive_msg(n, msg);
        //
        //     let count = vote_occurrences.entry(v.vote_to).or_insert(0);
        //     *count += 1;
        //
        // }
        //
        // let mut most_voted = 0;
        // let max_vote_count = 0;
        //
        // for (key, count) in vote_occurrences.iter(){
        //     if count > &max_vote_count {
        //         most_voted = *key;
        //     }
        // }
        //
        //
        //
        //
        //
        // return most_voted;
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

