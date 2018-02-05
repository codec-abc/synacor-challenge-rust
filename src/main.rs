mod opcode;
mod vm;

use std::fs::File;
use std::io::Read;
use std::io::Cursor;
use std::io;
extern crate byteorder;
use byteorder::{LittleEndian, ReadBytesExt};

fn main() 
{
    let file_name = "challenge.bin";
    let result = read_challenge_file(file_name);
    match result 
    {
        Err(e) => println!("Error while reading challenge file: {:?}", e.kind()),
        Ok(content) => 
        {
            //let mem_result = convert_to_u16_le(&content);
            let mem_result : Result<Vec<u8>,()> = Ok(content);
            match mem_result 
            {
                Err(e) => println!("File appear to be invalid: {:?}", e),
                Ok(mem) =>
                {
                    let mut vm = vm::VM::new(mem);
                    let mut result = vm.step();
                    let mut should_continue = true;
                    let mut input = String::new();

                    while should_continue && !result.is_err()
                    {
                        //println!("Type q or quit to exit:");
                        // io::stdin().read_line(&mut input).unwrap();
                        // let input_with_line_ending = input.to_lowercase();
                        // let word = input_with_line_ending.trim_right();

                        // should_continue = 
                        //     should_continue &&
                        //     word != "q" &&
                        //     word != "quit";

                        if !should_continue
                        {
                            continue;
                        }
                        
                        result = vm.step();
                    }
                    if result.is_err()
                    {
                        let err = result.unwrap_err();
                        println!("{:?}", err);
                    }
                }
            }
        },
    }
}

#[derive(Debug)]
enum ConvertToU16Error
{
    NotEvenNumberOfBytes
}

fn read_challenge_file(file_name: &str) -> 
    Result<Vec<u8>, std::io::Error>
{
    let mut file = File::open(file_name)?;
    let mut content : Vec<u8> = vec!();
    let result = file.read_to_end(&mut content);
    result.map(|_| content)
}
