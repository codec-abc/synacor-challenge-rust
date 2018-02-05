use opcode;
use opcode::*;
use std::result::Result;

pub struct VM
{
    memory : Vec<u8>,
    register : [u8; 8],
    stack : Vec<u16>,
    program_counter : u16,
    print_debug : bool
}

#[derive(Debug)]
pub enum RunFailure
{
    NotImplemented,
    OpCodeParseFailure(opcode::ReadOpCodeFailure),
    InvalidValue,
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
            print_debug : false,
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
            OpCode::Jump(jump) => self.handle_jump(jump),
            OpCode::JumpNotZero(jump_not_zero) => self.handle_jump_not_zero(jump_not_zero),
            OpCode::JumpZero(jump_zero) => self.handle_jump_zero(jump_zero),
            _ => Err(RunFailure::NotImplemented), 
        }
    }

    fn handle_out(&mut self, out : opcode::Out) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 4;
        print!("{}", (out.value as u8) as char);
        Ok(())
    }

    fn handle_jump(&mut self, jump : opcode::Jump) -> Result<(), RunFailure>
    {
        self.program_counter = jump.value * 2;
        Ok(())
    }

    fn handle_jump_not_zero(&mut self, jump_not_zero : opcode::JumpNotZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = check_number(jump_not_zero.value);
        match actual_value 
        {
            ParsedNumber::InvalidNumber => Err(RunFailure::InvalidValue),
            ParsedNumber::LiteralValue(val) =>
            {
                if jump_not_zero.value == 0 
                {
                    self.program_counter = self.program_counter + 6;
                }
                else
                {
                    self.program_counter = jump_not_zero.jump_location * 2;
                }
                Ok(())
            },
            ParsedNumber::Register(r) =>
            {
                let register_value = self.register[r as usize];
                if register_value == 0 
                {
                    self.program_counter = self.program_counter + 6;
                }
                else
                {
                    self.program_counter = jump_not_zero.jump_location * 2;
                }
                Ok(())
            },
        }

    }

    fn handle_jump_zero(&mut self, jump_zero : opcode::JumpZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = check_number(jump_zero.value);
        match actual_value 
        {
            ParsedNumber::InvalidNumber => Err(RunFailure::InvalidValue),
            ParsedNumber::LiteralValue(val) =>
            {
                if jump_zero.value != 0 
                {
                    self.program_counter = self.program_counter + 6;
                }
                else
                {
                    self.program_counter = jump_zero.jump_location * 2;
                }
                Ok(())
            },
            ParsedNumber::Register(r) =>
            {
                let register_value = self.register[r as usize];
                if register_value != 0 
                {
                    self.program_counter = self.program_counter + 6;
                }
                else
                {
                    self.program_counter = jump_zero.jump_location * 2;
                }
                Ok(())
            },
        }
    }

    fn handle_noop(&mut self) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 2;
        Ok(())
    }
}