enum ParsedNumber
{
    LiteralValue(u16),
    Register(u16),
    InvalidNumber,
}

fn check_number(number: u16) -> ParsedNumber
{
    if number < 32767
    {
        ParsedNumber::LiteralValue(number)
    }
    else if number <= 32775
    {
        ParsedNumber::Register(number - 32768)
    }
    else
    {
        ParsedNumber::InvalidNumber
    }
}

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