use crate::Registers;

pub struct CPU
{
    pc: u16,
    sp: u16,
    registers: Registers,
}

impl CPU
{
    pub fn new() -> CPU
    {
        CPU {pc: 0, sp: 0, registers: Registers::new()}
    }
}
