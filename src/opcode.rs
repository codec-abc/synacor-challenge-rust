use std;

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

enum ReadOpCodeFailure
{
    NotEnoughMemory,
    InvalidMemoryAddress,
    InvalidOpCode,
    InvalidOperandValue
}

fn read_memory_to_op_code(mem : &[u16], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let first_op_code_result = mem.get(offset as usize);
    if first_op_code_result.is_none()
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let first_op_code = *first_op_code_result.unwrap();
    let is_valid = check_number(first_op_code) != ParsedNumber::InvalidNumber;

    if !is_valid 
    {
        return Err(ReadOpCodeFailure::InvalidMemoryAddress);
    }

    match first_op_code
    {
        0 => Ok(OpCode::Halt) ,
        1 => handle_set_register_case(mem, offset),
        _ =>Err(ReadOpCodeFailure::InvalidOpCode),
    }  
}

fn handle_set_register_case(mem : &[u16], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let register_result = mem.get((offset + 1) as usize);
    let value_result = mem.get((offset + 2) as usize);
    let mut is_ok = register_result.is_some() && value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let register = *register_result.unwrap();
    let value = *value_result.unwrap();
    is_ok = 
        check_number(register).IsRegister() && 
        check_number(value).IsValidNumber();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let st = SetRegister {register : register, value : value };
    Ok(OpCode::SetRegister(st))
}

#[derive(PartialEq)]
pub enum ParsedNumber
{
    LiteralValue(u16),
    Register(u16),
    InvalidNumber,
}

impl ParsedNumber
{
    pub fn IsLiteralValue(&self) -> bool
    {
        match *self
        {
            ParsedNumber::LiteralValue(_) => true,
            _ => false
        }
    }

    pub fn IsRegister(&self) -> bool
    {
        match *self
        {
            ParsedNumber::Register(_) => true,
            _ => false
        }
    }

    pub fn IsInvalidNumber(&self) -> bool
    {
        match *self
        {
            ParsedNumber::InvalidNumber => true,
            _ => false
        }
    }

    pub fn IsValidNumber(&self) -> bool
    {
        match *self
        {
            ParsedNumber::InvalidNumber => false,
            _ => true
        }
    }
}
pub fn check_number(number: u16) -> ParsedNumber
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