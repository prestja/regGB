use crate::cpu::CPU;

type cpu_impl = fn(&mut CPU);

pub struct Instruction 
{
    pub desc: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub op: cpu_impl,
}

pub const INSTRUCTIONS: [Instruction; 1] = 
[
    Instruction 
    {
        desc: "nop",
        bytes: 1,
        cycles: 4,
        op: CPU::nop
    }
];
