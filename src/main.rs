mod registers;
mod cpu;
mod memory;
mod instruction;

use crate::registers::Registers;
use crate::cpu::CPU;
use crate::memory::Memory;
use crate::instruction::Instruction;

fn main() 
{
    println!("regGB started");
    let mut emulator = CPU::new();
    
    loop
    {
        emulator.execute();
    }
}
