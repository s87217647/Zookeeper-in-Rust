// use std::cell::RefCell;
// use std::collections::HashSet;
// use std::io::empty;
// use std::rc::{Rc, Weak};
// use crate::zookeeper::Zookeeper;
//
// pub struct Node {
//     pub id: i32,
//     pub zk : Rc<RefCell<Zookeeper>>,
//     status: String, // Leading, following, observing?
//     epoch: i32,
//     log : Vec<Message>, // tx history
//     buffer: Vec<Message>, // un processes tx, or communication in general, dk
//     data_dir: String,
//     data_base: HashSet<String> //imaginary data base, to store all the data
// }
//
//
// #[derive(Clone)]
// pub struct Message{
//     // All encompassing vessels
//     pub msg_type: String,
//     pub content: String
// }
// impl Message{
//
//
// }
//
//
//
// impl Node{
//     pub fn new(id: i32, zk: Rc<RefCell<Zookeeper>>) -> Node {
//         // let zk:Weak<RefCell<Zookeeper>> = Rc::downgrade(&Rc::new(RefCell::new(zk)));
//         Self{id, zk, status: String::new(), epoch: 0, log: Vec::new(), buffer: vec![], data_dir: String::new(), data_base: Default::default() }
//     }
//
// }
//
// impl Node {
//
//
//     pub fn receive_msg(n: &Node, msg: Message) -> Vote{
//         if msg.msg_type == "looking"{ // looking for a leader
//             if n.log.is_empty(){
//                 return Vote{vote_to: n.id, vote_from: n.id, zxid: Some(-1)};
//             }else {
//
//             }
//
//         }
//
//         return Vote{ vote_to: 0, vote_from: n.id, zxid: 0};
//     }
// }




use std::io::{self, Write};

struct Node;

impl Node {
    fn new() -> Self {
        Node
    }

    fn listen(&self) {
        println!("Node is ready to echo messages. Type 'exit' to quit.");

        loop {
            // Prompt user for input
            print!("> ");
            io::stdout().flush().unwrap(); // Ensure prompt is shown immediately

            // Read input from stdin
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read line");

            // Trim the input to remove any extra whitespace
            let input = input.trim();

            // Exit the loop if the user types "exit"
            if input.eq_ignore_ascii_case("exit") {
                println!("Shutting down the node.");
                break;
            }

            // Echo the input back
            println!("Echo: {}", input);
        }
    }
}

fn main() {
    let node = Node::new();
    node.listen();
}