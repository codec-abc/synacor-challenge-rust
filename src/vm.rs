use opcode;
use opcode::OpCode;
use std::result::Result;

pub struct VM
{
    memory : Vec<u16>,
    register : [u8; 8],
    stack : Vec<u16>,
    program_counter : u16
}

#[derive(Debug)]
pub enum RunFailure
{
    NotImplemented,
    OpCodeParseFailure(opcode::ReadOpCodeFailure),
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

    pub fn step(&mut self) -> 
        Result<(), RunFailure>
    {
        let op_code_result = 
            opcode::read_memory_to_op_code
            (
                &self.memory, 
                self.program_counter
            );

        match op_code_result
        {
            Err(e) => Err(RunFailure::OpCodeParseFailure(e)),
            Ok(op_code) => self.handle_op_code(op_code),
        }
    }

    pub fn handle_op_code(&mut self, op_code : opcode::OpCode) ->
        Result<(), RunFailure>
    {
        println!("{:?}", op_code);
        match op_code
        {
            OpCode::Noop => 
            {
                self.program_counter = self.program_counter + 1;
                Ok(())
            },
            _ =>
            {
                Err(RunFailure::NotImplemented)
            },
        }
    }
}