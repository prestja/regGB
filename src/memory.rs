pub struct Memory
{
    pub bytes: [u8; 65536]
}

impl Memory
{
    pub fn new () -> Memory 
    {
        Memory 
        {
            bytes: [0; 65536]
        }
    }

    pub fn from () -> Memory
    {
        let mem = Memory
        {
            bytes: [0; 65536]
        };
        return mem;
    }
}
