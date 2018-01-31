use opcode;

pub struct VM
{
    memory : Vec<u16>,
    register : [u8; 8],
    stack : Vec<u16>,
    program_counter : u16
}

impl VM
{
    pub fn new (memory_ : Vec<u16>) -> VM
    {
        VM 
        {
            memory : memory_,
            register : [0; 8], 
            stack : vec!(),
            program_counter : 0
        }
    }
}