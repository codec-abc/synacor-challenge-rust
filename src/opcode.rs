

struct SetRegister
{
    register : u16,
    value : u16,
}

pub enum OpCode
{
    Halt,
    SetRegister(SetRegister),
    Noop
}