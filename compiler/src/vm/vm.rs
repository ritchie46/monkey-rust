use crate::code::{read_be_u16, OpCode, Operand};
use crate::compiler::compiler::Bytecode;
use crate::err::VMError;
use monkey::eval::{evaluator::is_truthy, object::Object};
use std::borrow::{Borrow, BorrowMut};
use std::convert::TryFrom;
use std::mem;
use std::ptr::null;

const STACKSIZE: usize = 2048;
const OBJECT_TRUE: Object = Object::Bool(true);
const OBJECT_FALSE: Object = Object::Bool(false);
const OBJECT_NULL: Object = Object::Null;
const EMPTY_STACK: &'static str = "nothing on the stack";
const GLOBAL_SIZE: usize = 65536;
const MAX_FRAMES: usize = 1024;

#[derive(Clone)]
pub struct Frame {
    function_instr: Vec<u8>, // Object::CompiledFunction
    ip: usize,               // instruction pointer
    base_pointer: usize,
}

impl Frame {
    fn new(function_instr: Vec<u8>, base_pointer: usize) -> Frame {
        Frame {
            function_instr,
            ip: 0, // -1 not possible
            base_pointer,
        }
    }

    fn instructions(&self) -> &[u8] {
        &self.function_instr
    }
}

#[derive(Clone)]
pub struct VM<'cmpl> {
    pub constants: &'cmpl [Object],
    // pub globals: Vec<Object>,
    pub stack: Vec<Object>,
    pub sp: usize, // Stack Pointer: points to the next free registry on the stack
    pub frames: Vec<Frame>,
    pub frames_index: usize,
}

impl VM<'_> {
    pub fn new<'cmpl>(bytecode: &'cmpl Bytecode) -> VM<'cmpl> {
        let main_instructions = bytecode.instructions.to_vec();
        let main_frame = Frame::new(main_instructions, 0);
        let mut frames = Vec::with_capacity(MAX_FRAMES);
        frames.push(main_frame);

        VM {
            constants: bytecode.constants,

            stack: vec![OBJECT_NULL; STACKSIZE],
            sp: 0,
            frames,
            frames_index: 1,
        }
    }
}

impl<'cmpl> VM<'cmpl> {
    pub fn current_frame(&mut self) -> &mut Frame {
        self.frames.last_mut().unwrap()
    }

    pub fn current_instructions(&self) -> &[u8] {
        self.frames[self.frames_index - 1].instructions()
    }

    pub fn push_frame(&mut self, f: Frame) {
        self.frames.push(f);
        self.frames_index += 1
    }

    pub fn pop_frame(&mut self) -> Frame {
        self.frames_index -= 1;
        self.frames.pop().unwrap()
    }

    pub fn stack_top(&self) -> Option<&Object> {
        if self.sp == 0 {
            None
        } else {
            Some(&self.stack[self.sp - 1])
        }
    }

    pub fn pop(&mut self) -> Option<&Object> {
        if self.sp == 0 {
            return None;
        }
        self.sp -= 1;
        self.stack.get(self.sp)
    }

    pub fn pop_mut(&mut self) -> Option<&mut Object> {
        if self.sp == 0 {
            return None;
        }
        self.sp -= 1;
        self.stack.get_mut(self.sp)
    }

    /// Use raw pointers to get two multiple objects of the stack without cloning
    /// Needs unsafe code to dereference.
    pub fn pop_raw(&mut self) -> Option<*const Object> {
        if self.sp == 0 {
            return None;
        }
        let o = &self.stack[self.sp - 1] as *const Object;
        self.sp -= 1;
        Some(o)
    }

    /// Pop two references from the stack without cloning.
    /// The borrowck doesn't let use call self.pop twice wo/ a clone.
    pub fn pop_2(&mut self) -> Option<(&Object, &Object)> {
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

    pub fn push(&mut self, o: Object) -> Result<(), VMError> {
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

    fn build_array(&self, start_index: usize, end_index: usize) -> Object {
        let mut elements = Vec::with_capacity(end_index - start_index);

        for i in start_index..end_index {
            let el = self.stack[i].clone();
            elements.push(el)
        }
        Object::new_array(elements)
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

fn exec_prefix(right: &Object, oc: OpCode) -> Object {
    match oc {
        OpCode::Bang => match right {
            Object::Bool(v) => native_bool_to_object(!*v),
            Object::Int(i) => native_bool_to_object(!if *i == 0 { false } else { true }),
            Object::Null => OBJECT_TRUE,
            _ => Object::Error(format!("Prefix ! not allowed with {}", right.get_type())),
        },
        OpCode::Minus => match right {
            Object::Int(v) => Object::Int(-*v),
            _ => Object::Error(format!("Prefix - not allowed with {}", right.get_type())),
        },
        _ => panic!("unknown operator {:?}", oc),
    }
}

fn string_infix(left: &str, right: &str, oc: OpCode) -> Object {
    match oc {
        OpCode::Add => Object::String(format!("{}{}", left, right)),
        _ => Object::Error(format!("operand {:?} not support on string", oc)),
    }
}

pub fn run_vm(bc: &Bytecode) -> Result<Object, VMError> {
    let mut vm = VM::new(bc);
    let mut globals = vec![OBJECT_NULL; GLOBAL_SIZE];

    while vm.current_frame().ip < vm.current_frame().instructions().len() {
        let i = vm.current_frame().ip;
        let oc = unsafe { OpCode::from_unchecked(vm.current_instructions()[i]) };
        match oc {
            OpCode::Constant => {
                let (const_index, width) =
                    oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip += width;
                let r = vm.push(vm.constants[const_index].clone())?;
            }
            OpCode::Pop => {
                vm.pop();
            }
            OpCode::Add | OpCode::Sub | OpCode::Mul | OpCode::Div => {
                let (left, right) = vm.pop_2().expect(EMPTY_STACK);
                let result = match (left, right) {
                    (Object::Int(l), Object::Int(r)) => binary_operation(*l, *r, oc),
                    (Object::String(l), Object::String(r)) => string_infix(l, r, oc),
                    _ => panic!("not impl"),
                };
                vm.push(result);
            }
            OpCode::True => {
                vm.push(OBJECT_TRUE);
            }
            OpCode::False => {
                vm.push(OBJECT_FALSE);
            }
            OpCode::Equal | OpCode::NotEqual | OpCode::GT => {
                let result = {
                    // left and right should be dropped before getting 2nd mutable borrow.
                    let (left, right) = vm.pop_2().expect(EMPTY_STACK);
                    exec_cmp(left, right, oc)
                };
                vm.push(result);
            }
            OpCode::Minus | OpCode::Bang => {
                let result = {
                    let right = vm.pop().expect(EMPTY_STACK);
                    exec_prefix(right, oc)
                };
                vm.push(result);
            }
            OpCode::Jump => {
                // TODO: benchmark by directly reading big endian 16 here
                let (jump_pos, _) = oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip = jump_pos - 1;
            }
            OpCode::JumpNotTruthy => {
                let condition = vm.pop().expect(EMPTY_STACK);
                if !is_truthy(condition) {
                    let (jump_pos, width) =
                        oc.read_operand(&vm.current_instructions()[i + 1..]);
                    vm.current_frame().ip = jump_pos - 1;
                } else {
                    // skip jump operand
                    let width = oc.definition()[0];
                    vm.current_frame().ip += width;
                }
            }
            OpCode::Null => {
                vm.push(OBJECT_NULL);
            }
            OpCode::SetGlobal => {
                let (index, width) = oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip += width;
                globals[index] = vm.pop().expect(EMPTY_STACK).clone();
            }
            OpCode::GetGlobal => {
                let (index, width) = oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip += width;
                let global = globals[index].clone();
                vm.push(global);
            }
            OpCode::Array => {
                let (n_elements, width) =
                    oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip += width;
                let array = vm.build_array(vm.sp - n_elements, vm.sp);
                vm.push(array);
            }
            OpCode::Call => {
                let (n_args, width) =
                    oc.read_operand(&vm.current_instructions()[i + 1..]);
                vm.current_frame().ip += width;

                // We can replace the location on the stack because we know the function
                // get's popped off after execution
                let tmp = vm.stack.get_mut(vm.sp - 1 - n_args).unwrap();
                let mut fun = mem::replace(tmp, OBJECT_NULL);
                let fun = fun;

                if let Object::CompiledFunction {
                    instructions,
                    n_locals,
                } = fun
                {
                    let frame = Frame::new(instructions, vm.sp - n_args);

                    // leave space on the stack for local bindings
                    vm.sp = frame.base_pointer + n_locals;
                    vm.push_frame(frame);
                    // don't increment the instruction pointer this loop.
                    continue;
                } else {
                    panic!("calling non-function")
                }
            }
            OpCode::ReturnVal => {
                // TODO: Maybe use pop_and_own, but then last_popped does not work
                let return_value = vm.pop().expect(EMPTY_STACK).clone();
                // leave function scope
                let base_pointer = {
                    let frame = vm.pop_frame();
                    frame.base_pointer
                };
                vm.sp = base_pointer - 1;

                vm.push(return_value);
            }
            OpCode::Return => {
                let base_pointer = {
                    let frame = vm.pop_frame();
                    frame.base_pointer
                };
                vm.sp = base_pointer - 1;
                vm.push(OBJECT_NULL);
            }
            OpCode::SetLocal => {
                // Get index of local variable
                let (index, width) = oc.read_operand(&vm.current_instructions()[i + 1..]);

                // Keep track of the current position in the stack
                let base_pointer = {
                    let frame = vm.current_frame();
                    frame.ip += width;
                    frame.base_pointer
                };

                // Do a manual stack pop and swap the popped value w/ local binding
                vm.sp -= 1;
                vm.stack.swap(base_pointer + index, vm.sp);
            }
            OpCode::GetLocal => {
                // Get index of local variable
                let (index, width) = oc.read_operand(&vm.current_instructions()[i + 1..]);
                let base_pointer = {
                    let frame = vm.current_frame();
                    frame.ip += width;
                    frame.base_pointer
                };
                // Todo: benchmark clone
                let local = vm.stack.get(base_pointer + index).unwrap();
                vm.push(local.clone());
            }
            _ => panic!(format!("not impl {:?}", oc)),
        }
        vm.current_frame().ip += 1;
    }
    Ok(vm.last_popped().clone())
}
