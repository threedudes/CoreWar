use crate::types::{Instruction, Warrior, OpCode, Value};
use std::collections::HashMap;
use rand::prelude::*;

pub struct Pointer {
    position: i16,
    pub identifier: u8
}

pub struct Core {
    size: i16,
    pub data: HashMap<u16, Instruction>, // Using a HashMap instead of a vector to avoid storing 8000 values.
    pub pointers: Vec<Pointer>
}

impl Core {
    pub fn new(size: i16) -> Core {
        Core {
            data: HashMap::new(),
            size,
            pointers: vec![]
        }
    }
    pub fn tick(&mut self) {
        let mut toDelete = vec![];
        for (index, pointer) in self.pointers.iter_mut().enumerate() {
            println!("Running player {} instruction at place {}", pointer.identifier, pointer.position);
            let curr_state = self.data.clone();
            let instruction = curr_state.get(&(pointer.position as u16));
            let instruction = instruction.clone();
            let mut shouldDie = false;
            let pointer_move = match instruction {
                Some(instruction) => {
                    let (a,b) = &instruction.params;
                    let b = match b.value {
                        Value::Integer(i) => {pointer.position + i},
                        Value::Label(_) => {0}
                    };
                    match instruction.opcode {
                        OpCode::Mov => {
                            println!("Running mov in location {}",  pointer.position);
                            println!("Will move {b}");
                            self.data.insert((b  % self.size) as u16, instruction.clone());
                            1
                        }
                        _ => {1}

                    }
                },
                None => {
                    shouldDie = true;
                    0
                }
            } ;
            if shouldDie {
                toDelete.push(index)
            } else {
                pointer.position = (pointer.position + pointer_move) % self.size
            }
        }
        for i in toDelete {
            self.pointers.remove(i);
        }
    }

    pub fn load_warriors(&mut self, warriors: Vec<Warrior>) {
        let mut rng = thread_rng();
        let mut available_space = self.size;
        for warrior in &warriors {
            available_space -= warrior.instructions.len() as i16
        }
        let space_between = available_space / warriors.len() as i16;
        let first_place: u16 = rng.gen();
        let first_place = first_place % self.size as u16;
        let mut localisation = first_place as i16;
        for warrior in warriors {
            self.pointers.push(Pointer{identifier: warrior.identifier, position: localisation});
            for instruction in warrior.instructions {
                self.data.insert(localisation as u16, instruction);
                localisation += 1
            }
            localisation += space_between
        }
    }
}