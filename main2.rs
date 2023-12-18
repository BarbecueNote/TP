
use std::cell::{RefCell};
use crate::Exploration::UnExplored;
use std::rc::Rc;

enum Exploration {
    Explored, UnExplored, PartiallyExplored
}

struct Branch {
    label: String,
    left: Rc<RefCell<Maze>>,
    right: Rc<RefCell<Maze>>,
    status: Exploration
}

enum Maze {
    Branch(Branch),
    Leaf{label: String}
}
fn main(){
    let leaf2 = Rc::new(RefCell::new(Maze::Leaf{label: "2".to_string()}));
    let leaf4 = Rc::new(RefCell::new(Maze::Leaf{label: "4".to_string()}));
    let leaf5 = Rc::new(RefCell::new(Maze::Leaf{label: "5".to_string()}));
    let leaf8 = Rc::new(RefCell::new(Maze::Leaf{label: "8".to_string()}));
    let branch3 = Rc::new(RefCell::new(Maze::Branch(Branch{label: "3".to_string(), left: Rc::clone(&leaf4), right: Rc::clone(&leaf5), status: UnExplored})));
    let branch1 = Rc::new(RefCell::new(Maze::Branch(Branch{label: "1".to_string(), left: Rc::clone(&leaf2), right: Rc::clone(&branch3), status: UnExplored})));
    let branch7 = Rc::new(RefCell::new(Maze::Branch(Branch{label: "7".to_string(), left: Rc::clone(&leaf5), right: Rc::clone(&leaf8), status : UnExplored})));
    let branch6 = Rc::new(RefCell::new(Maze::Branch(Branch{label: "6".to_string(), left: Rc::clone(&branch3), right: Rc::clone(&branch7), status: UnExplored})));
    let branch0 = Rc::new(RefCell::new(Maze::Branch(Branch{label: "0".to_string(), left: Rc::clone(&branch1), right: Rc::clone(&branch6), status: UnExplored})));

    let mut work = vec![Rc::clone(&branch0)];
    let mut trace = vec![];
    while work.len() != 0 {
        let node = work.pop().expect("unexpected");
        node.borrow_mut().explore(Rc::clone(&node),&mut work, &mut trace);
        println!("trace so far: {:?}", trace);
    }
}
impl Maze {
    fn explore(&mut self, node: Rc<RefCell<Maze>>, work: &mut Vec<Rc<RefCell<Maze>>>,trace: &mut Vec<String>) {
        match self {
            Maze::Branch(b) =>
                match b.status {
                    Exploration::UnExplored => {
                        b.status = Exploration::PartiallyExplored;
                        trace.push(b.label.clone());
                        work.push(node);
                        b.left.borrow_mut().explore(Rc::clone(&b.left),work, trace);
                    }
                    Exploration::PartiallyExplored => {
                        b.status = Exploration::Explored;
                        b.right.borrow_mut().explore(Rc::clone(&b.right),work, trace);
                    }
                    Exploration::Explored => trace.push(b.label.clone())
                }
            Maze::Leaf{label} => trace.push(String::from(label.clone()))
        }
    }
}

