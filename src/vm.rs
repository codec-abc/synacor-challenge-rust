use opcode;
use opcode::*;
use std::result::Result;

pub struct VM
{
    memory : Vec<u16>,
    register : [u16; 8],
    stack : Vec<u16>,
    program_counter : u16,
    step_nb : u64,
    print_debug : bool
}

#[derive(Debug)]
pub enum RunFailure
{
    Halt,
    NotImplemented(opcode::OpCode),
    OpCodeParseFailure(opcode::ReadOpCodeFailure),
    InvalidValue,
    CannotPopStackIsEmpty,
    CannotReturnStackIsEmpty,
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
            program_counter : 0,
            step_nb : 0,
            print_debug : true,
        }
    }

    fn get_literal_value_or_register_value(&self, number : u16) -> 
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
                let val = self.register[r as usize];
                self.get_literal_value_or_register_value(val)
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
            println!("step {}", self.step_nb);

            println!
            (
                "current program counter {} in bytes 0x{}",
                self.program_counter, 
                format!("{:X}", self.program_counter * 2)
            );

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

            // println!
            // (   "[{}, {}, {}, {}, {}, {}, {}, {}]", 
            //     format!("0x{:X}", self.register[0]),
            //     format!("0x{:X}", self.register[1]),
            //     format!("0x{:X}", self.register[2]),
            //     format!("0x{:X}", self.register[3]),
            //     format!("0x{:X}", self.register[4]),
            //     format!("0x{:X}", self.register[5]),
            //     format!("0x{:X}", self.register[6]),
            //     format!("0x{:X}", self.register[7]),
            // );

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

            // print!("[");
            // is_first = true;
            // for val in &self.stack
            // {
            //     if !is_first
            //     {
            //         print!(", {}", format!("0x{:X}", val));
            //     }
            //     else
            //     {
            //         print!("{}", format!("0x{:X}", val));
            //         is_first = false;
            //     }
            // }
            // println!("]");

            println!("current op code {:?}", op_code_result);

        }
        match op_code_result
        {
            Err(e) => Err(RunFailure::OpCodeParseFailure(e)),
            Ok(op_code) => 
            {
                let result = self.handle_op_code(op_code);
                self.step_nb = self.step_nb + 1;
                result
            },
        }
    }

    pub fn handle_op_code(&mut self, op_code : opcode::OpCode) ->
        Result<(), RunFailure>
    {
        if self.print_debug
        {
            //println!("{:?}", op_code);
            println!("");
        }
        match op_code
        {
            OpCode::Halt => Err(RunFailure::Halt),
            OpCode::SetRegister(set_register) => self.handle_set_register(set_register),
            OpCode::Push(push) => self.handle_push(push),
            OpCode::Pop(pop) => self.handle_pop(pop),
            OpCode::IsEqual(is_equal) => self.handle_is_equal(is_equal),
            OpCode::IsGreaterThan(greater_than) => self.handle_greater_than(greater_than),
            OpCode::Jump(jump) => self.handle_jump(jump),
            OpCode::JumpNotZero(jump_not_zero) => self.handle_jump_not_zero(jump_not_zero),
            OpCode::JumpZero(jump_zero) => self.handle_jump_zero(jump_zero),
            OpCode::Add(add) => self.handle_add(add),
            OpCode::Multiply(multiply) => self.handle_multiply(multiply),
            OpCode::Modulo(modulo) => self.handle_modulo(modulo),
            OpCode::And(and) => self.handle_and(and),
            OpCode::Or(or) => self.handle_or(or),
            OpCode::Not(not) => self.handle_not(not),
            OpCode::ReadMemory(read_memory) => self.handle_read_memory(read_memory),
            OpCode::WriteMemory(write_memory) => self.handle_write_memory(write_memory),
            OpCode::Call(call) => self.handle_call(call),
            OpCode::Return => self.handle_return(),
            OpCode::Out(out) => self.handle_out(out),
            OpCode::Noop => self.handle_noop(),
            _ => Err(RunFailure::NotImplemented(op_code)), 
        }
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
                self.program_counter = self.program_counter + 3;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue)
        } 
    }

    fn handle_push(&mut self, push : opcode::Push) -> Result<(), RunFailure>
    {
        let val = self.get_literal_value_or_register_value(push.value)?;
        self.stack.push(val);
        self.program_counter = self.program_counter + 2;
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
                    self.program_counter = self.program_counter + 2;
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
        let b = self.get_literal_value_or_register_value(is_equal.first_operand)?;
        let c = self.get_literal_value_or_register_value(is_equal.second_operand)?;

        let actual_value = check_number(is_equal.cell_result);
        match actual_value
        {
           ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = if b == c { 1 } else { 0 };
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_greater_than(&mut self, is_greater_than : opcode::IsGreaterThan) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(is_greater_than.first_operand)?;
        let c = self.get_literal_value_or_register_value(is_greater_than.second_operand)?;

        let actual_value = check_number(is_greater_than.cell_result);
        match actual_value
        {
           ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = if b > c { 1 } else { 0 };
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_jump(&mut self, jump : opcode::Jump) -> Result<(), RunFailure>
    {
        self.program_counter = jump.value;
        Ok(())
    }

    fn handle_jump_not_zero(&mut self, jump_not_zero : opcode::JumpNotZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = self.get_literal_value_or_register_value(jump_not_zero.value)?;
        if actual_value == 0 
        {
            self.program_counter = self.program_counter + 3;
        }
        else
        {
            self.program_counter = jump_not_zero.jump_location;
        }
        Ok(())
    }

    fn handle_jump_zero(&mut self, jump_zero : opcode::JumpZero) -> 
        Result<(), RunFailure>
    {
        let actual_value = self.get_literal_value_or_register_value(jump_zero.value)?;
        if actual_value != 0 
        {
            self.program_counter = self.program_counter + 3;
        }
        else
        {
            self.program_counter = jump_zero.jump_location;
        }
        Ok(())
    }

    fn handle_add(&mut self, add : opcode::Add) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(add.first_operand)?;
        let c = self.get_literal_value_or_register_value(add.second_operand)?;

        assert!(check_number(b).is_literal_value());
        assert!(check_number(c).is_literal_value());

        let actual_value = check_number(add.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = (b + c) % 32768; // overflow ?
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_multiply(&mut self, multiply : opcode::Multiply) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(multiply.first_operand)? as u64;
        let c = self.get_literal_value_or_register_value(multiply.second_operand)? as u64;

        assert!(check_number(b as u16).is_literal_value());
        assert!(check_number(c as u16).is_literal_value());

        let actual_value = check_number(multiply.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = ((b * c) % 32768) as u16; // overflow ?
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_modulo(&mut self, modulo : opcode::Modulo) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(modulo.first_operand)? as u64;
        let c = self.get_literal_value_or_register_value(modulo.second_operand)? as u64;

        assert!(check_number(b as u16).is_literal_value());
        assert!(check_number(c as u16).is_literal_value());

        let actual_value = check_number(modulo.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = ((b % c) % 32768) as u16; // overflow ?
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_and(&mut self, add : opcode::And) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(add.first_operand)?;
        let c = self.get_literal_value_or_register_value(add.second_operand)?;

        assert!(check_number(b).is_literal_value());
        assert!(check_number(c).is_literal_value());

        let actual_value = check_number(add.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = b & c;
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_or(&mut self, or : opcode::Or) -> Result<(), RunFailure>
    {
        let b = self.get_literal_value_or_register_value(or.first_operand)?;
        let c = self.get_literal_value_or_register_value(or.second_operand)?;

        assert!(check_number(b).is_literal_value());
        assert!(check_number(c).is_literal_value());

        let actual_value = check_number(or.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = b | c;
                self.program_counter = self.program_counter + 4;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_not(&mut self, not : opcode::Not) -> Result<(), RunFailure>
    {
        let val = self.get_literal_value_or_register_value(not.operand)?;

        assert!(check_number(val).is_literal_value());

        let actual_value = check_number(not.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r) =>
            {
                self.register[r as usize] = (!val) & 0b0111_1111_1111_1111;
                self.program_counter = self.program_counter + 3;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_read_memory(&mut self, read_memory : opcode::ReadMemory) 
        -> Result<(), RunFailure>
    {
        let mut mem_cell = check_number(read_memory.memory_address_to_read);
        let actual_value = check_number(read_memory.cell_result);
        match actual_value
        {
            ParsedNumber::Register(r_dest) =>
            {
                match mem_cell
                {
                    ParsedNumber::Register(r) =>
                    {
                        let mem_address = self.register[r as usize];
                        if !(check_number(mem_address).is_literal_value())
                        {
                            panic!("Error in read memory implementation");
                        }
                        let value = self.memory[mem_address as usize];
                        self.register[r_dest as usize] = value;
                        self.program_counter = self.program_counter + 3;
                        Ok(())
                    },
                    ParsedNumber::LiteralValue(val) =>
                    {
                        self.register[r_dest as usize] = self.memory[val as usize] ;
                        self.program_counter = self.program_counter + 3;
                        Ok(())
                    },
                    _ => Err(RunFailure::InvalidValue) 

                }
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_write_memory(&mut self, write_memory: opcode::WriteMemory)
        -> Result<(), RunFailure>
    {
        let memory_address_to_write_to = check_number(write_memory.memory_address_to_write_to);
        let value_to_write = self.get_literal_value_or_register_value(write_memory.value)?;
        //print!("{}", (value_to_write as u8) as char);
        match memory_address_to_write_to
        {
            ParsedNumber::Register(r) =>
            {
                let mem_address = self.register[r as usize];
                if !(check_number(mem_address).is_literal_value())
                {
                    panic!("Error in write memory implementation");
                }
                self.memory[mem_address as usize] = value_to_write;
                self.program_counter = self.program_counter + 3;
                Ok(())
            },
            ParsedNumber::LiteralValue(val) =>
            {
                self.memory[val as usize] = value_to_write ;
                self.program_counter = self.program_counter + 3;
                Ok(())
            },
            _ => Err(RunFailure::InvalidValue) 
        }
    }

    fn handle_call(&mut self, call : opcode::Call) -> Result<(), RunFailure>
    {
        self.stack.push(self.program_counter + 2);
        let actual_value = self.get_literal_value_or_register_value(call.value)?;
        self.program_counter = actual_value;
        Ok(())
    }

    fn handle_return(&mut self) -> Result<(), RunFailure>
    {
        if self.stack.len() < 1
        {
            Err(RunFailure::CannotReturnStackIsEmpty)
        }
        else
        {
            let return_address = self.stack.pop().unwrap();
            self.program_counter = return_address;
            Ok(())
        }
    }

    fn handle_out(&mut self, out : opcode::Out) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 2;
        print!("{}", (out.value as u8) as char);
        Ok(())
    }

    fn handle_noop(&mut self) -> Result<(), RunFailure>
    {
        self.program_counter = self.program_counter + 1;
        Ok(())
    }
}