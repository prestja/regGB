use crate::Registers;
use crate::Memory;

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

    pub fn tick(&self)
    {
        
    }
}
