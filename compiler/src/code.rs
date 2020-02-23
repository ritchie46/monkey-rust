use num_enum::{TryFromPrimitive, UnsafeFromPrimitive};
use std::collections::HashMap;
use std::convert::From;
use std::convert::TryInto;
use std::fmt::Write;

pub type Instructions = Vec<u8>;
pub type Operand = usize;

#[derive(
    PartialEq, Hash, Eq, Copy, Clone, Debug, UnsafeFromPrimitive, TryFromPrimitive,
)]
#[repr(u8)]
pub enum OpCode {
    Constant,      // 0 Operand: constants pool location
    Add,           // 1 No operand. Take two values from the stack.
    Pop,           // 2 Pop last element from stack. No operands.
    Sub,           // 3 No operand. Take two values from the stack.
    Mul,           // 4 No operand. Take two values from the stack.
    Div,           // 5 No operand. Take two values from the stack.
    True,          // 6
    False,         // 7
    Equal,         // 8 No operand. Take two values from the stack.
    NotEqual,      // 9 No operand. Take two values from the stack.
    GT,            // 10 No operand. Take two values from the stack.
    Minus,         // 11 '-' prefix. No operands. One value from the stack.
    Bang,          // 12 '!' prefix. No operands. One value from the stack.
    JumpNotTruthy, // 13 Operand: jump offset.
    Jump,          // 14 Operand: jump offset.
    Null,          // 15 No operand.
    SetGlobal,     // 16 Operand: index. One value from the stack.
    GetGlobal,     // 17 Operand: index.
    Array,         // 18 Operand: Number of elements in the array.
    Call,          // 19 No operand. Function is on the stack.
    ReturnVal,     // 20 No operand. One value from the stack.
    Return,        // 21 No operand.
    SetLocal,      // 22 Operand: index. One value from the stack.
    GetLocal,      // 23 Operand: index.
}

impl OpCode {
    fn as_byte(&self) -> u8 {
        *self as u8
    }
    pub fn definition(&self) -> &'static [usize] {
        use OpCode::*;
        match self {
            Constant | JumpNotTruthy | Jump | SetGlobal | GetGlobal | Array => &[2],
            SetLocal | GetLocal => &[1],
            _ => &[], // all opcodes wo/ operands
        }
    }

    pub fn make(&self, operands: &[Operand]) -> Instructions {
        let mut instr = self.as_byte().to_be_bytes().to_vec();

        let op_width = self.definition();
        for (i, operand) in operands.iter().enumerate() {
            let width = op_width[i];
            match width {
                2 => instr.extend_from_slice(&(*operand as u16).to_be_bytes()),
                1 => instr.push(*operand as u8),
                _ => panic!("not impl"),
            }
        }
        instr
    }

    /// instructions slice starting just behind the OpCode
    /// returns operand and width of operand
    pub fn read_operand(&self, instructions: &[u8]) -> (Operand, usize) {
        use OpCode::*;
        match self {
            Constant | JumpNotTruthy | Jump | SetGlobal | GetGlobal | Array => {
                (read_be_u16(&instructions[..2]) as usize, 2)
            }
            SetLocal | GetLocal => (instructions[0] as usize, 1),
            _ => panic!("no operand after opcode!"),
        }
    }
}

pub fn read_operands(op_width: &[usize], ins: &[u8]) -> (Vec<Operand>, usize) {
    let mut operands = vec![];
    let mut offset = 1; // first one is opcode
    for (i, width) in op_width.iter().enumerate() {
        match width {
            2 => operands.push(read_be_u16(&ins[offset..]) as usize),
            1 => operands.push(ins[offset] as usize),
            _ => panic!("not impl"),
        }
        offset += *width;
    }
    (operands, offset)
}

fn fmt_disassemble(ins: &[u8]) -> String {
    let mut s = "".to_string();
    let mut c = 0;
    while c < ins.len() {
        let opcode = unsafe { OpCode::from_unchecked(ins[c]) };
        let (operands, n_read) = read_operands(&opcode.definition(), &ins[c..]);
        writeln!(&mut s, "{:04} opcode: {:?} {:?}", c, opcode, operands);

        c += n_read;
    }
    s
}

pub fn read_be_u16(input: &[u8]) -> u16 {
    let (int_bytes, rest) = input.split_at(std::mem::size_of::<u16>());
    u16::from_be_bytes(int_bytes.try_into().unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    fn fmt_instructions(opcodes: &[OpCode], operands: &[&[Operand]]) -> String {
        let mut instr = vec![];

        for (oc, op) in opcodes.iter().zip(operands) {
            instr.extend_from_slice(&oc.make(op));
        }
        fmt_disassemble(&instr)
    }

    #[test]
    fn test_opconstant() {
        let operand = 65534;
        assert_eq!([0, 255, 254], OpCode::Constant.make(&[operand])[..]);

        let ins = OpCode::Constant.make(&[operand]);
        let r = read_operands(&OpCode::Constant.definition(), &ins);
        assert_eq!(operand, r.0[0]);

        let s = fmt_instructions(
            &[OpCode::Constant, OpCode::Constant, OpCode::Constant],
            &[&[1], &[2], &[65534]],
        );

        assert_eq!(
            r#"0000 opcode: Constant [1]
0003 opcode: Constant [2]
0006 opcode: Constant [65534]
"#,
            s
        )
    }

    #[test]
    fn test_opadd() {
        let s = fmt_instructions(
            &[OpCode::Add, OpCode::Constant, OpCode::Constant],
            &[&[], &[2], &[65534]],
        );
        assert_eq!(
            "0000 opcode: Add []
0001 opcode: Constant [2]
0004 opcode: Constant [65534]
",
            s
        )
    }
}
