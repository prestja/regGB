use crate::Registers;
use crate::Memory;
use crate::instruction::{Instruction, INSTRUCTIONS};

pub struct CPU
{
    pc: u16,
    sp: u16,
    registers: Registers,
    memory: Memory
}

impl CPU
{
    pub fn new() -> CPU
    {
        CPU {pc: 0, sp: 0, registers: Registers::new(), memory: Memory::new()}
    }

    pub fn execute(&mut self)
    {
        let b0 = self.memory.bytes[self.pc as usize];
        let next = &INSTRUCTIONS[b0 as usize];
        (next.op)(self);

        println!("pc: {} instr: {}", self.pc, next.desc);
        self.pc = self.pc.wrapping_add(1 + next.bytes as u16);
    }

    pub fn nop(&mut self)
    {
    }
}
