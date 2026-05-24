use rand::{Rng, RngExt};
use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;


pub struct RobotFactory {
    used_names: Rc<RefCell<HashSet<String>>>,
}

pub struct Robot {
    name: String,
    used_names: Rc<RefCell<HashSet<String>>>,
}

impl RobotFactory {
    pub fn new() -> Self {
        Self {
            used_names: Rc::new(RefCell::new(HashSet::new())),
        }
    }

    pub fn new_robot<R: Rng>(&mut self, _rng: &mut R) -> Robot {
        Robot {
            name: generate_unique_names(_rng, &self.used_names),
            used_names: Rc::clone(&self.used_names),
        }
    }
}

impl Robot {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn reset<R: Rng>(&mut self, _rng: &mut R) {
        let old_name = self.name.clone();
        self.name = generate_unique_names(_rng, &self.used_names);
        self.used_names.borrow_mut().remove(&old_name);
    }
}

fn generate_unique_names<R: Rng>(_rng: &mut R, used_names: &RefCell<HashSet<String>>) -> String {
    let mut name = String::new();

    loop {
        name.clear();
        name.push(_rng.random_range(b'A'..=b'Z') as char);
        name.push(_rng.random_range(b'A'..=b'Z') as char);
        name.push(_rng.random_range(b'0'..=b'9') as char);
        name.push(_rng.random_range(b'0'..=b'9') as char);
        name.push(_rng.random_range(b'0'..=b'9') as char);

        if used_names.borrow_mut().insert(name.clone()) {
            break;
        } 
    }

    name
}





