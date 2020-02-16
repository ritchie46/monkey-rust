use std::collections::HashMap;
use std::convert::From;
use std::convert::TryInto;
use std::fmt::Write;

pub type Instructions = Vec<u8>;
pub type Operand = usize;

struct Definition {
    name: String,
    op_width: Vec<u8>,
}

impl Definition {
    fn new(name: &str, op_width: Vec<u8>) -> Definition {
        Definition {
            name: name.to_string(),
            op_width,
        }
    }
}

#[derive(PartialEq, Hash, Eq, Copy, Clone, Debug)]
pub enum OpCode {
    Constant, // operand: constants pool location
}

impl OpCode {
    fn as_byte(&self) -> u8 {
        *self as u8
    }
    fn definition(&self) -> Definition {
        match self {
            OpCode::Constant => Definition::new("opconstant", vec![2]),
        }
    }

    pub fn make(&self, operands: &[Operand]) -> Instructions {
        let mut instr = self.as_byte().to_be_bytes().to_vec();

        let def = self.definition();

        for (i, operand) in operands.iter().enumerate() {
            let width = def.op_width[i];
            match width {
                2 => instr.extend_from_slice(&(*operand as u16).to_be_bytes()),
                _ => panic!("not impl"),
            }
        }
        instr
    }
}

impl From<u8> for OpCode {
    fn from(byte: u8) -> Self {
        match byte {
            0 => OpCode::Constant,
            _ => panic!("not impl"),
        }
    }
}

fn read_operands(def: Definition, ins: &[u8]) -> (Vec<Operand>, usize) {
    let mut operands = vec![];
    let mut offset = 1; // first one is opcode
    for (i, width) in def.op_width.iter().enumerate() {
        match width {
            2 => operands.push(read_be_u16(&ins[offset..]) as usize),
            _ => panic!("not impl"),
        }
        offset += *width as usize;
    }
    (operands, offset)
}

fn fmt_disassemble(ins: &[u8]) -> String {
    let mut s = "".to_string();
    let mut c = 0;
    while c < ins.len() {
        let opcode = OpCode::from(ins[c]);
        let (operands, n_read) = read_operands(opcode.definition(), &ins[c..]);
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

    #[test]
    fn test_opconstant() {
        let operand = 65534;
        assert_eq!([0, 255, 254], OpCode::Constant.make(&[operand])[..]);

        let ins = OpCode::Constant.make(&[operand]);
        let r = read_operands(OpCode::Constant.definition(), &ins);
        assert_eq!(operand, r.0[0]);

        let mut instr = vec![];

        instr.extend_from_slice(&OpCode::Constant.make(&[1]));
        instr.extend_from_slice(&OpCode::Constant.make(&[2]));
        instr.extend_from_slice(&OpCode::Constant.make(&[65534]));

        let s = fmt_disassemble(&instr);

        assert_eq!(
            r#"0000 opcode: Constant [1]
0003 opcode: Constant [2]
0006 opcode: Constant [65534]
"#,
            s
        )
    }
}
