use std::io::{Error, ErrorKind};
#[derive(Clone, Debug)]

pub enum OpCode {
    Move,
    Jump,
    Dat,
}


impl OpCode {
    pub fn from_opstring(opstring: &str) -> Result<OpCode, Error> {
        match opstring {
            "mov" => Ok(OpCode::Move),
            "jmp" => Ok(OpCode::Jump),
            "dat" => Ok(OpCode::Dat),
            _ => Err(Error::new(ErrorKind::InvalidData, format!("{} instruction not found !", opstring)))
        }
    }
}


#[derive(Clone, Debug)]
pub struct Param {
    pub mode: AddressingModes,
    pub value: Value
}

#[derive(Clone, Debug)]
pub enum Value {
    Integer (i16),
    Label (String)
}

#[derive(Clone,Debug)]
pub enum Modifier {
    A,
    B,
    AB,
    BA,
    F,
    X,
    I
}

impl Modifier {
    pub fn from_str(data: &str) -> Option<Modifier> {
        match data{
            "a" => Some(Modifier::A),
            "b" => Some(Modifier::B),
            "ab" => Some(Modifier::AB),
            "ba" => Some(Modifier::BA),
            "f" => Some(Modifier::F),
            "x" => Some(Modifier::X),
            "i" => Some(Modifier::I),
            _ => None
        }

    }
}

#[derive(Copy, Clone, Debug)]
pub enum AddressingModes {
    Immediate, // #
    Direct, // $
    AIndirect, // *
    BIndirect, // @
    AIndirectPredecrement, // {
    BIndirectPredecrement, // <
    AIndirectPostincrement, // }
    BIndirectPostincrement // >
}

impl AddressingModes {
    pub fn from_str(data: &str) -> AddressingModes {
        match data {
            "#" => AddressingModes::Immediate,
            "*" => AddressingModes::AIndirect,
            "@" => AddressingModes::BIndirect,
            "{" => AddressingModes::AIndirectPredecrement,
            "<" => AddressingModes::BIndirectPredecrement,
            "}" => AddressingModes::AIndirectPostincrement,
            ">" => AddressingModes::BIndirectPostincrement,
            _ => AddressingModes::Direct
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    pub modifier: Option<Modifier>,
    pub opcode: OpCode,
    pub params: (Param, Param) // A and B
}
