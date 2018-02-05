use opcode;
use opcode::*;
use std::result::Result;

pub struct VM
{
    memory : Vec<u8>,
    register : [u16; 8],
    stack : Vec<u16>,
    program_counter : u16,
    print_debug : bool
}

#[derive(Debug)]
pub enum RunFailure
{
    NotImplemented(opcode::OpCode),
    OpCodeParseFailure(opcode::ReadOpCodeFailure),
    InvalidValue,
    CannotPopStackIsEmpty,
}

impl VM
{
    pub fn new (memory_ : Vec<u8>) -> VM
    {
        VM 
        {
            memory : memory_,
            register : [0; 8], 
            stack : vec!(),
            program_counter : 0,
            print_debug : true,
        }
    }

    fn get_memory_cell_or_register_value(&self, number : u16) -> 
        Result<u16, RunFailure>
    {
        let actual_value = check_number(number);
        match actual_value 
        {
            ParsedNumber::InvalidNumber => Err(RunFailure::InvalidValue),
            ParsedNumber::LiteralValue(val) =>
            {
                Ok(val)
            },
            ParsedNumber::Register(r) =>
            {
                Ok(self.register[r as usize])
            }
        }
    }

    pub fn step(&mut self) -> 
        Result<(), RunFailure>
    {
        let op_code_result = 
            opcode::read_memory_to_op_code
            (
                &self.memory,
                self.program_counter
            );

        if self.print_debug
        {
            println!("current op code {:?}", op_code_result);
            println!("current program counter 0x{}", format!("{:X}", self.program_counter));
            println!
            (   "[{}, {}, {}, {}, {}, {}, {}, {}]", 
                self.register[0],
                self.register[1],
                self.register[2],
                self.register[3],
                self.register[4],
                self.register[5],
                self.register[6],
                self.register[7],
            );
            print!("[");
            let mut is_first = true;
            for val in &self.stack
            {
                if !is_first
                {
                    print!(", {}", val);
                }
                else
                {
                    print!("{}", val);
                    is_first = false;
                }
            }
            println!("]");
            println!("");
        }
        match op_code_result
        {
            Err(e) => Err(RunFailure::OpCodeParseFailure(e)),
            Ok(op_code) => self.handle_op_code(op_code),
        }
    }

    pub fn handle_op_code(&mut self, op_code : opcode::OpCode) ->
        Result<(), RunFailure>
    {
        if self.print_debug
        {
            println!("{:?}", op_code);
        }
        match op_code
        {
            OpCode::Out(out) => self.handle_out(out),
            OpCode::Noop => self.handle_noop(),
            OpCode::SetRegister(set_register) => self.handle_set_register(set_register),
            OpCode::Jump(jump) => self.handle_jump(jump),
            OpCode::JumpNotZero(jump_not_zero) => self.handle_jump_not_zero(jump_not_zero),
            OpCode::JumpZero(jump_zero) => self.handle_jump_zero(jump_zero),
            OpCode::Add(add) => self.handle_add(add),
            OpCode::IsEqual(is_equal) => self.handle_is_equal(is_equal),
            OpCode::Push(push) => self.handle_push(push),
            OpCode::Pop(pop) => self.handle_pop(pop),
            _ => Err(RunFailure::NotImplemented(op_code)), 
        }
    }

    fn handle_push(&mut self, push : opcode::Push) -> Result<(), RunFailure>
    {
        let val = self.get_memory_cell_or_register_value(push.value)?;
        self.stack.push(val);
        self.program_counter = self.program_counter + 4;
        Ok(())
    }

    fn handle_pop(&mut self, pop : opcode::Pop) -> Result<(), RunFailure>
    {
        if self.stack.len() > 0
        {
            let stack_value = self.stack.pop().unwrap();
            let actual_value = check_number(pop.value);
            match actual_value
            {
                ParsedNumber::Register(r) =>
                {
                    self.program_counter = self.program_counter + 4;
                    self.register[r as usize] = stack_value;
                    Ok(())
                },
                _ => Err(RunFailure::InvalidValue)
            }
        }
        else
        {
            Err(RunFailure::CannotPopStackIsEmpty)
        }
    }

    fn handle_is_equal(&mut self, is_equal : opcode::IsEqual) -> Result<(), RunFailure>
    {
        let b = self.get_memory_cell_or_register_value(is_equal.first_operand)?;
        let c = self.get_memory_cell_or_register_value(is_equal.second_operand)?;

        let actual_value = check_number(is_equal.cell_result);
        match actual_value
        {
           ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = if b == c { 1 } else { 0 }; // overflow ?
                self.program_counter = self.program_counter + 8;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_jump(&mut self, jump : opcode::Jump) -> Result<(), RunFailure>
    {
        self.program_counter = jump.value * 2;
        Ok(())
    }

    fn handle_set_register(&mut self, set_register : opcode::SetRegister) ->
        Result<(), RunFailure>
    {
        let actual_value = check_number(set_register.register);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = set_register.value;
                self.program_counter = self.program_counter + 6;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue)
        } 
    }

    fn handle_jump_not_zero(&mut self, jump_not_zero : opcode::JumpNotZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = self.get_memory_cell_or_register_value(jump_not_zero.value)?;
        if actual_value == 0 
        {
            self.program_counter = self.program_counter + 6;
        }
        else
        {
            self.program_counter = jump_not_zero.jump_location * 2;
        }
        Ok(())
    }

    fn handle_jump_zero(&mut self, jump_zero : opcode::JumpZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = self.get_memory_cell_or_register_value(jump_zero.value)?;
        if actual_value != 0 
        {
            self.program_counter = self.program_counter + 6;
        }
        else
        {
            self.program_counter = jump_zero.jump_location * 2;
        }
        Ok(())
    }

    fn handle_add(&mut self, add : opcode::Add) -> Result<(), RunFailure>
    {
        let b = self.get_memory_cell_or_register_value(add.first_operand)?;
        let c = self.get_memory_cell_or_register_value(add.second_operand)?;

        let actual_value = check_number(add.cell_result);
        match actual_value
        {
           ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = (b + c) % 32768; // overflow ?
                self.program_counter = self.program_counter + 8;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_out(&mut self, out : opcode::Out) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 4;
        print!("{}", (out.value as u8) as char);
        Ok(())
    }

    fn handle_noop(&mut self) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 2;
        Ok(())
    }
}