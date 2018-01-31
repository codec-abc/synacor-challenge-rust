

struct SetRegister
{
    register : u16,
    value : u16,
}

struct Push
{
    value : u16,
}

struct Pop
{
    value : u16,
}

struct IsEqual
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct GreaterThan
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct Jump
{
    value : u16,
}

struct JumpNotZero
{
    value: u16,
    jump_location : u16,
}

struct JumpZero
{
    value: u16,
    jump_location : u16,
}

struct Add
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct Multiply
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct Modulo
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct And
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct Or
{
    cell_result : u16,
    first_operand : u16,
    second_operand : u16,
}

struct Not
{
    cell_result : u16,
    operand : u16,
}

struct ReadMemory
{
    cell_result : u16,
    memory_address_to_read : u16,
}

struct WriteMemory
{
    value : u16,
    memory_address_to_write_to : u16,
}

struct Call
{
    value : u16,
}

struct Out
{
    value : u16,
}

struct In
{
    value : u16,
}

pub enum OpCode
{
    Halt,
    SetRegister(SetRegister),
    Push(Push),
    Pop(Pop),
    IsEqual(IsEqual),
    GreaterThan(GreaterThan),
    Jump(Jump),
    JumpNotZero(JumpNotZero),
    JumpZero(JumpZero),
    Add(Add),
    Multiply(Multiply),
    Modulo(Modulo),
    And(And),
    Or(Or),
    Not(Not),
    ReadMemory(ReadMemory),
    WriteMemory(WriteMemory),
    Call(Call),
    Return,
    In(In),
    Out(Out),
    Noop,
}