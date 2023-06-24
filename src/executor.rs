use std::collections::HashMap;

pub struct Executor {
    event_map: HashMap<u32, Box<dyn FnMut(&mut Self)>>,
    event_map_once: HashMap<u32, Box<dyn FnOnce(&mut Self)>>
}

impl Executor {
    pub fn new() -> Self {
        Self { event_map: HashMap::new(), event_map_once: HashMap::new()}
    }

    pub fn insert_keep(&mut self, event_id: u32, fun: Box<dyn FnMut(&mut Self)>) {
        self.event_map.insert(event_id, fun);
    }

    pub fn insert_once(&mut self, event_id: u32, fun: Box<dyn FnOnce(&mut Self)>) {
        self.event_map_once.insert(event_id, fun);
    }

    pub fn run(&mut self, event_id: u32) {
        if let Some(mut fun) = self.event_map.remove(&event_id) {
            fun(self);
            self.event_map.insert(event_id, fun);
        } else {
            if let Some(fun) = self.event_map_once.remove(&event_id) {
                fun(self);
            }
        }
    }


}