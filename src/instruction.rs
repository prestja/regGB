pub struct Instruction 
{
    pub desc: &'static str,
    pub bytes: u8,
    pub cycles: u8,
    pub op: fn()
}

pub fn nop()
{
    
}

pub const INSTRUCTIONS: [Instruction; 1] = 
[
    Instruction 
    {
        desc: "nop",
        bytes: 0,
        cycles: 4,
        op: nop
    }
];
