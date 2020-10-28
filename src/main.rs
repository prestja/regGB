mod registers;
mod cpu;
mod memory;

use crate::registers::Registers;
use crate::cpu::CPU;
use crate::memory::Memory;

fn main() 
{
    println!("regGB started");
    let emulator = CPU::new();
    
    loop
    {
        emulator.tick();
    }
}
