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
            let mem_result = convert_to_u16_le(&content);
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

fn convert_to_u16_le(mem : &Vec<u8>) -> 
    Result<Vec<u16>, ConvertToU16Error>
{
    let size = mem.len();
    if size % 2 != 0 
    {
        Err(ConvertToU16Error::NotEvenNumberOfBytes)
    } 
    else
    {
        let mut mem_u16 : Vec<u16> = vec!();
        let mut rdr = Cursor::new(mem);
        let mut keep_going = true;
        while keep_going
        {
            let u16_result = rdr.read_u16::<LittleEndian>();
            if u16_result.is_err() 
            {
                keep_going = false;
            }
            else
            {
                mem_u16.push(u16_result.unwrap());
            }
        }
        Ok(mem_u16)
    }
}

fn read_challenge_file(file_name: &str) -> 
    Result<Vec<u8>, std::io::Error>
{
    let mut file = File::open(file_name)?;
    let mut content : Vec<u8> = vec!();
    let result = file.read_to_end(&mut content);
    result.map(|_| content)
}
