#[derive(Debug)]
pub struct SetRegister
{
    pub register : u16,
    pub value : u16,
}

#[derive(Debug)]
pub struct Push
{
    pub value : u16,
}

#[derive(Debug)]
pub struct Pop
{
    pub value : u16,
}

#[derive(Debug)]
pub struct IsEqual
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct IsGreaterThan
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct Jump
{
    pub value : u16,
}

#[derive(Debug)]
pub struct JumpNotZero
{
    pub value: u16,
    pub jump_location : u16,
}

#[derive(Debug)]
pub struct JumpZero
{
    pub value: u16,
    pub jump_location : u16,
}

#[derive(Debug)]
pub struct Add
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct Multiply
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct Modulo
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct And
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct Or
{
    pub cell_result : u16,
    pub first_operand : u16,
    pub second_operand : u16,
}

#[derive(Debug)]
pub struct Not
{
    pub cell_result : u16,
    pub operand : u16,
}

#[derive(Debug)]
pub struct ReadMemory
{
    pub memory_address_to_read : u16,
    pub cell_result : u16,
}

#[derive(Debug)]
pub struct WriteMemory
{
    pub value : u16,
    pub memory_address_to_write_to : u16,
}

#[derive(Debug)]
pub struct Call
{
    pub value : u16,
}

#[derive(Debug)]
pub struct Out
{
    pub value : u16,
}

#[derive(Debug)]
pub struct In
{
    pub value : u16,
}

#[derive(Debug)]
pub enum OpCode
{
    Halt,
    SetRegister(SetRegister),
    Push(Push),
    Pop(Pop),
    IsEqual(IsEqual),
    IsGreaterThan(IsGreaterThan),
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

#[derive(Debug)]
pub enum ReadOpCodeFailure
{
    NotEnoughMemory,
    InvalidMemoryAddress,
    InvalidOpCode,
    InvalidOperandValue
}

fn read_mem_as_u16_le(mem : &[u8], offset : u16) -> Option<u16>
{
    let low_byte_maybe = mem.get(offset as usize);
    let high_byte_maybe = mem.get((offset + 1) as usize);

    if low_byte_maybe.is_none() || high_byte_maybe.is_none()
    {
        None
    }
    else
    {
        let high_byte : u16 = *high_byte_maybe.unwrap() as u16;
        let low_byte : u16 = *low_byte_maybe.unwrap() as u16;
        
        Some(high_byte.rotate_left(8) + low_byte)
    }
}

pub fn read_memory_to_op_code(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let first_op_code_result = read_mem_as_u16_le(mem, offset);
    if first_op_code_result.is_none()
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let first_op_code = first_op_code_result.unwrap();
    let is_valid = check_number(first_op_code) != ParsedNumber::InvalidNumber;

    if !is_valid 
    {
        return Err(ReadOpCodeFailure::InvalidMemoryAddress);
    }

    match first_op_code
    {
        0 => Ok(OpCode::Halt) ,
        1 =>  handle_set_register_case(mem, offset),
        2 =>  handle_push_case(mem, offset),
        3 =>  handle_pop_case(mem, offset),
        4 =>  handle_is_equal_case(mem, offset),
        5 =>  handle_is_greater_case(mem, offset),
        6 =>  handle_jump_case(mem, offset),
        7 =>  handle_jump_not_zero_case(mem, offset),
        8 =>  handle_jump_zero_case(mem, offset),
        9 =>  handle_add_case(mem, offset),
        10 => handle_multiply_case(mem,offset),
        11 => handle_modulo_case(mem,offset),
        12 => handle_and_case(mem, offset),
        13 => handle_or_case(mem, offset),
        14 => handle_not_case(mem, offset),
        15 => handle_read_memory_case(mem, offset),
        16 => handle_write_memory_case(mem, offset),
        17 => handle_call_case(mem, offset),
        18 => Ok(OpCode::Return),
        19 => handle_out_case(mem, offset),
        20 => handle_in_case(mem, offset),
        21 => Ok(OpCode::Noop),
        x => {
            println!("Invalid OpCode for value {}", x);
            Err(ReadOpCodeFailure::InvalidOpCode)
        },
    }  
}

fn handle_set_register_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let register_result = read_mem_as_u16_le(mem, (offset + 2));
    let value_result = read_mem_as_u16_le(mem, (offset + 4));
    let mut is_ok = register_result.is_some() && value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let register = register_result.unwrap();
    let value = value_result.unwrap();
    is_ok = 
        check_number(register).is_register() && 
        check_number(value).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let st = SetRegister {register : register, value : value };
    Ok(OpCode::SetRegister(st))
}

fn handle_push_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, (offset + 2));
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_valid_number();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let push = Push {value : value};
    Ok(OpCode::Push(push))
}

fn handle_pop_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, (offset + 2));
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_valid_number();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let pop = Pop {value : value};
    Ok(OpCode::Pop(pop))
}

fn handle_is_equal_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, (offset + 2));
    let first_operand_result = read_mem_as_u16_le(mem, (offset + 4));
    let second_operand_result = read_mem_as_u16_le(mem, (offset + 6));
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let is_equal = 
        IsEqual 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::IsEqual(is_equal))
}

fn handle_is_greater_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let is_greater = 
        IsGreaterThan 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::IsGreaterThan(is_greater))
}

fn handle_jump_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_valid_number();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let jump = Jump {value : value * 2};
    Ok(OpCode::Jump(jump))
}

fn handle_jump_not_zero_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let jump_location_result = read_mem_as_u16_le(mem, offset + 4);
    let mut is_ok = value_result.is_some() && jump_location_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    let jump_location = jump_location_result.unwrap();
    is_ok = 
        check_number(value).is_valid_number() && 
        check_number(jump_location).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let jump_not_zero = JumpNotZero {value : value, jump_location : jump_location * 2 };
    Ok(OpCode::JumpNotZero(jump_not_zero))
}

fn handle_jump_zero_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let jump_location_result = read_mem_as_u16_le(mem, offset + 4);
    let mut is_ok = value_result.is_some() && jump_location_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    let jump_location = jump_location_result.unwrap();
    is_ok = 
        check_number(value).is_valid_number() && 
        check_number(jump_location).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let jump_zero = JumpZero {value : value, jump_location : jump_location * 2 };
    Ok(OpCode::JumpZero(jump_zero))
}

fn handle_add_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let add = 
        Add 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::Add(add))
}

fn handle_multiply_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let mult = 
        Multiply 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::Multiply(mult))
}

fn handle_modulo_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let modulo = 
        Modulo 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::Modulo(modulo))
}

fn handle_and_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let and = 
        And 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::And(and))
}

fn handle_or_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let first_operand_result = read_mem_as_u16_le(mem, offset + 4);
    let second_operand_result = read_mem_as_u16_le(mem, offset + 6);
    let mut is_ok = 
        cell_result.is_some() && 
        first_operand_result.is_some() &&
        second_operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let first_operand = first_operand_result.unwrap();
    let second_operand = second_operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(first_operand).is_valid_number() &&
        check_number(second_operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let or = 
        Or 
        {
            cell_result : cell, 
            first_operand : first_operand, 
            second_operand : second_operand 
        };
    Ok(OpCode::Or(or))
}

fn handle_not_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let operand_result = read_mem_as_u16_le(mem, offset + 4);
    let mut is_ok = 
        cell_result.is_some() && 
        operand_result.is_some();

    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let cell = cell_result.unwrap();
    let operand = operand_result.unwrap();
    is_ok = 
        check_number(cell).is_valid_number() && 
        check_number(operand).is_valid_number();
    
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let not = 
        Not 
        {
            cell_result : cell, 
            operand : operand
        };
    Ok(OpCode::Not(not))
}

fn handle_read_memory_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let cell_result = read_mem_as_u16_le(mem, offset + 2);
    let memory_address_to_read_result = read_mem_as_u16_le(mem, offset + 4);
    
    let mut is_ok = 
        cell_result.is_some() &&
        memory_address_to_read_result.is_some();
        
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    
    let cell = cell_result.unwrap();
    let memory_address_to_read = memory_address_to_read_result.unwrap();

    is_ok = 
        check_number(cell).is_valid_number() &&
        check_number(memory_address_to_read).is_literal_value();
        
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let read_memory = 
        ReadMemory 
        {
            cell_result : cell, 
            memory_address_to_read : memory_address_to_read
        };
    Ok(OpCode::ReadMemory(read_memory))
}

fn handle_write_memory_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let memory_address_to_write_to_result = read_mem_as_u16_le(mem, offset + 2);
    let value_result = read_mem_as_u16_le(mem, offset + 4);
    
    let mut is_ok = 
        value_result.is_some() &&
        memory_address_to_write_to_result.is_some();
        
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    
    let value = value_result.unwrap();
    let memory_address_to_write_to = memory_address_to_write_to_result.unwrap();

    is_ok = 
        check_number(value).is_valid_number() &&
        check_number(memory_address_to_write_to).is_literal_value();
        
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let write_memory = 
        WriteMemory 
        {
            value : value, 
            memory_address_to_write_to : memory_address_to_write_to
        };
    Ok(OpCode::WriteMemory(write_memory))
}

fn handle_call_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_literal_value();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let call = Call {value : value};
    Ok(OpCode::Call(call))
}

fn handle_out_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_valid_number();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let out = Out {value : value};
    Ok(OpCode::Out(out))
}

fn handle_in_case(mem : &[u8], offset : u16) -> 
    Result<OpCode, ReadOpCodeFailure>
{
    let value_result = read_mem_as_u16_le(mem, offset + 2);
    let mut is_ok = value_result.is_some();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::NotEnoughMemory);
    }
    let value = value_result.unwrap();
    is_ok = check_number(value).is_valid_number();
    if !is_ok
    {
        return Err(ReadOpCodeFailure::InvalidOperandValue);
    }
    let in_ = In {value : value};
    Ok(OpCode::In(in_))
}


#[derive(PartialEq)]
#[derive(Eq)]
pub enum ParsedNumber
{
    LiteralValue(u16),
    Register(u16),
    InvalidNumber,
}

impl ParsedNumber
{
    pub fn is_literal_value(&self) -> bool
    {
        match *self
        {
            ParsedNumber::LiteralValue(_) => true,
            _ => false
        }
    }

    pub fn is_register(&self) -> bool
    {
        match *self
        {
            ParsedNumber::Register(_) => true,
            _ => false
        }
    }

    pub fn is_invalid_number(&self) -> bool
    {
        match *self
        {
            ParsedNumber::InvalidNumber => true,
            _ => false
        }
    }

    pub fn is_valid_number(&self) -> bool
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