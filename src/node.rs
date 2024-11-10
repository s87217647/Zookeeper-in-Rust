use std::io::empty;
use crate::zookeeper::Zookeeper;

pub struct Message{
    msg_type: String, //
    content: String
}
pub struct Node {
    id: i32,
    status: String, // Leading, following, observing?
    epoch: i32,
    log : Vec<Message>, // to log unprocessed info
    data_dir: String,
    zk: &'Zookeeper // dual direction communication, aye?
}

impl Node {
    pub fn new(id: i32, zk: &Zookeeper) -> Node {
        Self{id, zk, status: String::new(), epoch: 0, log: Vec::new(), data_dir: String::new()}
    }

}


// enum
// struct Node {
//     id: i32,
//     hisotry: vec<mes>
//
// }

