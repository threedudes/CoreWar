use crate::macros;
use std::collections::HashMap;



macros::matcher_gen!{
    enum OpCode {
        "mov" => Mov,
        "jmp" => Jump,
        "dat" => Dat,
        "equ" => Equ,
        "add" => Add,
        "jmz" => Jmz,
        "for" => For, //not a valid instruction
        "rof" => Rof, // same
        "gate" => Gate,
        "dec" => Dec,
        "spl" => Spit,
        "djn" => Djn,
        "jmn" => Jmn
    }
}

macros::matcher_gen!{
    enum Modifier {
        "a" => A,
        "b" => B,
        "ab" => AB,
        "ba" => BA,
        "f" => F,
        "x" => X,
        "i" => I
    }
}

macros::matcher_gen!{
    enum AddressingMode {
        "#" => Immediate,
        "$" => Direct,
        "*" => AIndirect,
        "@" => BIndirect,
        "{" => AIndirectPredecrement,
        "<" => BIndirectPredecrement,
        "}" => AIndirectPostincrement,
        ">" => BIndirectPostincrement
    }
}

#[derive(Clone, Debug)]
pub struct Param {
    pub mode: AddressingMode,
    pub value: Value
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer (i16),
    Label (String)
}


#[derive(Clone, Debug)]
pub struct Instruction {
    pub modifier: Option<Modifier>,
    pub opcode: OpCode,
    pub params: (Param, Param), // A and B
}

#[derive(Debug)]
pub struct Warrior {
    pub instructions: Vec<Instruction>,
    pub identifier: u8
}

impl Warrior {
    pub fn process_labels(&mut self, labels: HashMap<String, i16>) {
        //TODO: Handle label not found error
        for (index, instruction) in self.instructions.iter_mut().enumerate() {
            let (a,b) = &mut instruction.params;
            a.value = match &a.value {
                Value::Integer(i) => Value::Integer(*i),
                Value::Label(lab) => Value::Integer(labels[lab] - index as i16)
            };
            b.value = match &b.value {
                Value::Integer(i) => Value::Integer(*i),
                Value::Label(lab) => Value::Integer(labels[lab] - index as i16)
            }
        }
    }
}