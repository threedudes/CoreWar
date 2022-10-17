use crate::types::{Instruction, Warrior};
use std::collections::HashMap;
use rand::prelude::*;
pub struct Core {
    size: u16,
    pub data: HashMap<u16, Instruction> // Using a HashMap instead of a vector to avoid storing 8000 values. 
}

impl Core {
    pub fn new(size: u16) -> Core {
        Core {
            data: HashMap::new(),
            size
        }
    }
    pub fn load_warriors(&mut self, warriors: Vec<Warrior>) {
        let mut rng = thread_rng();
        let mut available_space = self.size;
        for warrior in &warriors {
            available_space -= warrior.instructions.len() as u16
        }
        let space_between = available_space / warriors.len() as u16;
        let first_place: u16 = rng.gen();
        let first_place = first_place % self.size;
        let mut localisation = first_place;
        for warrior in warriors {
            for instruction in warrior.instructions {
                self.data.insert(localisation, instruction);
                localisation += 1
            }
            localisation += space_between
        }
    }
}