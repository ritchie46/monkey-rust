use crate::code::{read_be_u16, OpCode};
use crate::compiler::compiler::Bytecode;
use crate::err::VMError;
use monkey::eval::object::Object;
use std::convert::TryFrom;

const STACKSIZE: usize = 2048;
const OBJECT_TRUE: Object = Object::Bool(true);
const OBJECT_FALSE: Object = Object::Bool(false);
const OBJECT_NULL: Object = Object::Null;

pub struct VM<'cmpl> {
    constants: &'cmpl [Object],
    instructions: &'cmpl [u8],

    pub stack: Vec<Object>,
    sp: usize, // Points to the next free registry on the stack
}

impl VM<'_> {
    pub fn new<'cmpl>(bytecode: &'cmpl Bytecode) -> VM<'cmpl> {
        VM {
            constants: bytecode.constants,
            instructions: bytecode.instructions,
            stack: vec![Object::Null; STACKSIZE],
            sp: 0,
        }
    }
    fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    fn pop(&mut self) -> Option<&Object> {
        if self.sp == 0 {
            return None;
        }
        let o = &self.stack[self.sp - 1];
        self.sp -= 1;
        Some(o)
    }

    /// Use raw pointers to get two multiple objects of the stack without cloning
    /// Needs unsafe code to dereference.
    fn pop_raw(&mut self) -> Option<*const Object> {
        if self.sp == 0 {
            return None;
        }
        let o = &self.stack[self.sp - 1] as *const Object;
        self.sp -= 1;
        Some(o)
    }

    /// Pop two references from the stack without cloning.
    /// The borrowck doesn't let use call self.pop twice wo/ a clone.
    fn pop_2(&mut self) -> Option<(&Object, &Object)> {
        if self.sp <= 1 {
            return None;
        }
        // first right than left. Such that this function can be unpacked as left, right
        let two = unsafe {
            let r = &*self.pop_raw().unwrap();
            let l = &*self.pop_raw().unwrap();
            (l, r)
        };
        Some(two)
    }

    fn push(&mut self, o: Object) -> Result<(), VMError> {
        if self.sp >= STACKSIZE {
            return Err(VMError::StackOverflow);
        }
        self.stack[self.sp] = o;
        self.sp += 1;
        Ok(())
    }

    pub fn last_popped(&self) -> &Object {
        &self.stack[self.sp]
    }

    pub fn run(&mut self) -> Result<(), VMError> {
        let mut i = 0;
        while i < self.instructions.len() {
            let op = unsafe { OpCode::from_unchecked(self.instructions[i]) };
            match op {
                OpCode::Constant => {
                    let const_index = read_be_u16(&self.instructions[i + 1..]) as usize;
                    i += 2;
                    let r = self.push(self.constants[const_index].clone())?;
                }
                OpCode::Pop => {
                    self.pop();
                }
                OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                    let (left, right) = self.pop_2().expect("nothing on the stack");
                    let result = match (left, right) {
                        (Object::Int(l), Object::Int(r)) => binary_operation(*l, *r, op),
                        _ => panic!("not impl"),
                    };
                    self.push(result);
                }
                OpCode::True => {
                    self.push(OBJECT_TRUE);
                }
                OpCode::False => {
                    self.push(OBJECT_FALSE);
                }
                OpCode::Equal | OpCode::NotEqual | OpCode::GT => {
                    let result = {
                        // left and right should be dropped before getting 2nd mutable borrow.
                        let (left, right) = self.pop_2().expect("nothing on the stack");
                        exec_cmp(left, right, op)
                    };
                    self.push(result);
                }
                OpCode::Minus | OpCode::Bang => {
                    let result = {
                        let right = self.pop().expect("nothing on the stack");
                        exec_prefix(right, op)
                    };
                    self.push(result);
                }
                _ => panic!(format!("not impl {:?}", op)),
            }
            i += 1;
        }
        Ok(())
    }
}

fn binary_operation(l: i64, r: i64, op: OpCode) -> Object {
    match op {
        OpCode::Add => Object::Int(l + r),
        OpCode::Sub => Object::Int(l - r),
        OpCode::Mul => Object::Int(l * r),
        OpCode::Div => Object::Int(l / r),
        _ => panic!("not impl"),
    }
}

fn exec_cmp(left: &Object, right: &Object, op: OpCode) -> Object {
    match (left, right) {
        (Object::Int(l), Object::Int(r)) => exec_int_cmp(*l, *r, op),
        (Object::Bool(l), Object::Bool(r)) => exec_bool_cmp(*l, *r, op),
        _ => panic!("NOT IMPL"),
    }
}

fn exec_int_cmp(left: i64, right: i64, op: OpCode) -> Object {
    match op {
        OpCode::Equal => native_bool_to_object(left == right),
        OpCode::GT => native_bool_to_object(left > right),
        OpCode::NotEqual => native_bool_to_object(left != right),
        _ => panic!("unknown operator {:?}", op),
    }
}

fn exec_bool_cmp(left: bool, right: bool, op: OpCode) -> Object {
    match op {
        OpCode::Equal => native_bool_to_object(left == right),
        OpCode::NotEqual => native_bool_to_object(left != right),
        _ => panic!("unknown operator {:?}", op),
    }
}

fn native_bool_to_object(input: bool) -> Object {
    if input {
        OBJECT_TRUE
    } else {
        OBJECT_FALSE
    }
}

fn exec_prefix(right: &Object, op: OpCode) -> Object {
    match op {
        OpCode::Bang => match right {
            Object::Bool(v) => native_bool_to_object(!*v),
            Object::Int(i) => native_bool_to_object(!if *i == 0 { false } else { true }),
            _ => Object::Error(format!("Prefix ! not allowed with {}", right.get_type())),
        },
        OpCode::Minus => match right {
            Object::Int(v) => Object::Int(-*v),
            _ => Object::Error(format!("Prefix - not allowed with {}", right.get_type())),
        },
        _ => panic!("unknown operator {:?}", op),
    }
}
