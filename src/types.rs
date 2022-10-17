use crate::macros;

macros::matcher_gen!{
    enum OpCode {
        "mov" => Mov,
        "jmp" => Jump
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
    pub params: (Param, Param), // A and B,
    pub label: String
}
