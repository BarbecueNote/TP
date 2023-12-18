use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, PartialEq)]
enum Exploration {
    Explored,
    UnExplored,
}

enum Maze {
    Branch {
        label: String,
        left: Rc<RefCell<Maze>>,
        right: Rc<RefCell<Maze>>,
        status: RefCell<Exploration>,
    },
    Leaf {
        label: String,
    },
}

impl Maze {
    fn explore(&self, trace: &mut Vec<String>) {
        match self {
            Maze::Branch { label, left, right, status } => {
                let mut is_unexplored = false;
                {
                    let mut status_borrow = status.borrow_mut();
                    if *status_borrow == Exploration::UnExplored {
                        *status_borrow = Exploration::Explored;
                        is_unexplored = true;
                    }
                }

                if is_unexplored {
                    trace.push(label.clone());
                    left.borrow().explore(trace);
                    right.borrow().explore(trace);
                } else {
                    trace.push(label.clone());
                }
            }
            Maze::Leaf { label } => {
                trace.push(label.clone());
            }
        }
    }
}


fn main() {
    let leaf2 = Rc::new(RefCell::new(Maze::Leaf { label: "2".to_string() }));
    let leaf4 = Rc::new(RefCell::new(Maze::Leaf { label: "4".to_string() }));
    let leaf5 = Rc::new(RefCell::new(Maze::Leaf { label: "5".to_string() }));
    let leaf8 = Rc::new(RefCell::new(Maze::Leaf { label: "8".to_string() }));

    let branch3 = Rc::new(RefCell::new(Maze::Branch {
        label: "3".to_string(),
        left: leaf4.clone(),
        right: leaf5.clone(),
        status: RefCell::new(Exploration::UnExplored),
    }));

    let branch1 = Rc::new(RefCell::new(Maze::Branch {
        label: "1".to_string(),
        left: leaf2.clone(),
        right: branch3.clone(),
        status: RefCell::new(Exploration::UnExplored),
    }));

    let branch7 = Rc::new(RefCell::new(Maze::Branch {
        label: "7".to_string(),
        left: leaf5.clone(),
        right: leaf8.clone(),
        status: RefCell::new(Exploration::UnExplored),
    }));

    let branch6 = Rc::new(RefCell::new(Maze::Branch {
        label: "6".to_string(),
        left: branch3.clone(),
        right: branch7.clone(),
        status: RefCell::new(Exploration::UnExplored),
    }));

    let branch0 = Rc::new(RefCell::new(Maze::Branch {
        label: "0".to_string(),
        left: branch1.clone(),
        right: branch6.clone(),
        status: RefCell::new(Exploration::UnExplored),
    }));

    let mut trace = Vec::new();
    branch0.borrow().explore(&mut trace);
    println!("{:?}", trace);
}
