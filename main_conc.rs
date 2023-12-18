use std::cell::{Ref, RefCell};
use std::sync::{Arc, Mutex};
use crate::Exploration::UnExplored;
use std::rc::Rc;
use std::thread;
use std::time::Duration;

enum Exploration {
    Explored, UnExplored, PartiallyExplored
}
struct Branch {
    label: String,
    left: Arc<Mutex<Maze>>,
    right: Arc<Mutex<Maze>>,
    status: Exploration
}
enum Maze {
    Branch(Branch),
    Leaf{label: String, exit: bool}
}
impl Maze {
    fn explore(&mut self, node: Arc<Mutex<Maze>>, work: &mut Vec<Arc<Mutex<Maze>>>,trace: &mut Vec<String>)-> bool {
        match self {
            Maze::Branch(b) =>
                match b.status {
                    Exploration::UnExplored => {
                        b.status = Exploration::PartiallyExplored;
                        trace.push(b.label.clone());
                        work.push(node);
                        b.left.lock().unwrap().explore(Arc::clone(&b.left),work, trace)
                    }
                    Exploration::PartiallyExplored => {
                        b.status = Exploration::Explored;
                        b.right.lock().unwrap().explore(Arc::clone(&b.right),work, trace)
                    }
                    Exploration::Explored => {
                        trace.push(b.label.clone());
                        return false;
                    }
                }
            Maze::Leaf{label, exit} => {
                trace.push(String::from(label.clone()));
                if *exit {return true} else {return false}
            }
        }
    }
}

fn main(){
    let leaf2 = Arc::new(Mutex::new(Maze::Leaf{label: "2".to_string(), exit:false}));
    let leaf4 = Arc::new(Mutex::new(Maze::Leaf{label: "4".to_string(), exit: false}));
    let leaf5 = Arc::new(Mutex::new(Maze::Leaf{label: "5".to_string(), exit: true}));
    let leaf8 = Arc::new(Mutex::new(Maze::Leaf{label: "8".to_string(), exit: false}));
    let branch3 = Arc::new(Mutex::new(Maze::Branch(Branch{label: "3".to_string(), left: Arc::clone(&leaf4), right: Arc::clone(&leaf5), status: UnExplored})));
    let branch1 = Arc::new(Mutex::new(Maze::Branch(Branch{label: "1".to_string(), left: Arc::clone(&leaf2), right: Arc::clone(&branch3), status: UnExplored})));
    let branch7 = Arc::new(Mutex::new(Maze::Branch(Branch{label: "7".to_string(), left: Arc::clone(&leaf5), right: Arc::clone(&leaf8), status : UnExplored})));
    let branch6 = Arc::new(Mutex::new(Maze::Branch(Branch{label: "6".to_string(), left: Arc::clone(&branch3), right: Arc::clone(&branch7), status: UnExplored})));
    let branch0 = Arc::new(Mutex::new(Maze::Branch(Branch{label: "0".to_string(), left: Arc::clone(&branch1), right: Arc::clone(&branch6), status: UnExplored})));
    let mut handles = vec![];
    let mut i=Arc::new(Mutex::new(0));
    let mut work = Arc::new(Mutex::new(vec![Arc::clone(&branch0)]));
    let mut cond = Arc::new(Mutex::new(0));

    while work.lock().unwrap().len() != 0 {
        let work_ = Arc::clone(&work);
        let i_ = Arc::clone(&i);
        let cond_ = Arc::clone(&cond);
        if *cond_.lock().unwrap() == 1 {break};
        let handle = thread::spawn(move || {
            let mut j = i_.lock().unwrap();
            let mut condition = cond_.lock().unwrap();
            *j+=1;
            let mut trace: Vec<String> = vec![];
            let node = work_.lock().unwrap().pop().expect("unexpected");
            let res = node.lock().unwrap().explore(Arc::clone(&node), &mut work_.lock().unwrap(), &mut trace);
            println!("worker {} explored nodes: {:?}", j, trace);
            match res {
                false => (),
                true => *condition=1
            }
        });
        thread::sleep(Duration::new(0, 1000));
        thread::yield_now();

        handles.push(handle);
    }
    for handle in handles {
        handle.join().unwrap();
    }
}
