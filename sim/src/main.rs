#![allow(unused)]


use std::env;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;
 
#[macro_use]
extern crate maplit;

pub fn main() {

    
    
let args: Vec<String> = env::args().skip(1).collect(); //The skip here stops the executable binary being placed in the output

println!("{:?}", args);

//This just ensures the correct number of command line arguments are provided and provides the user with a usage guide if incorrect
//The max amount of arguments (if using the verbose and or helper flag) is 9 whereas the minimum is 8
if args.len() < 8 || args.len() > 9 { 
    println!("Usage: [-hv] -s <s> -E <E> -b <b> -t <tracefile>");
    return;
}

let index_of_s = args.iter().position(|find_s: &String| find_s == "-s").unwrap();

#[allow(non_snake_case)] //Although I know this goes against variable naming conventions as we are dealing with a capital E I think it is justified to defy these conventions for clarity
let index_of_E = args.iter().position(|find_E: &String| find_E == "-E").unwrap();

let index_of_b = args.iter().position(|find_b: &String| find_b == "-b").unwrap();
let index_of_t = args.iter().position(|find_t: &String| find_t == "-t").unwrap();

let value_of_s: &String = &args[index_of_s + 1];
#[allow(non_snake_case)]
let value_of_E:&String = &args[index_of_E + 1];
let value_of_b:&String = &args[index_of_b + 1];
let path_to_trace:&String = &args[index_of_t + 1];



println!("Index of -s string: {}, Number value of s: {}",index_of_s, value_of_s);

println!("Index of -E string: {}, Number value of E: {}",index_of_E, value_of_E);

println!("Index of -b string: {}, Number value of b: {}",index_of_b, value_of_b);

println!("Index of -t string: {}, Path to trace file: {}",index_of_t, path_to_trace);

#[allow(non_snake_case)]
//This dynamically calculates how many BYTES the combined cache is
fn calculate_cache_size(s: &String, E:&String, b:&String) -> u32{
    let int_of_s: u32= s.parse().unwrap(); //This is the amount of Sets 
    let int_of_E: u32= E.parse().unwrap(); //This is the amount of Cache Lines per set 
    let int_of_b: u32= b.parse().unwrap(); //This is the block offset (the amount of bits that determine which byte to take from the requested block)

    let cache_bytes_size: u32 = (2_u32.pow(int_of_s)) * (2_u32.pow(int_of_b)) * int_of_E;

    cache_bytes_size

}

fn make_file_line_separated_vector(filepath: &str) -> Vec<String>{
    let file = File::open(filepath).expect("Failed to open file");
    let reader = BufReader::new(file);
    let mut vec_of_lines: Vec<String> = vec![];
    for line in reader.lines(){
        let line_to_add = line.unwrap();
        vec_of_lines.push(line_to_add);
    }
    return vec_of_lines;
}


fn convert_from_hex_to_binary(hex_address: &str) -> Result<String, &'static str>{ //The function either returns a String or an error (as handled by &'static str) 
//Just for future reference, the reason you chose a static lifetime is so the error message/string persists for the whole time the program executes as we might need to see at the last second that one of the strings failed to convert to binary
    let my_map_of_hex_to_binary: HashMap<char, String> = hashmap! {
        '0' => "0000".to_string(),
        '1' => "0001".to_string(),
        '2' => "0010".to_string(),
        '3' => "0011".to_string(),
        '4' => "0100".to_string(),
        '5' => "0101".to_string(),
        '6' => "0110".to_string(),
        '7' => "0111".to_string(),
        '8' => "1000".to_string(),
        '9' => "1001".to_string(),
        'a' => "1010".to_string(),
        'b' => "1011".to_string(),
        'c' => "1100".to_string(),
        'd' => "1101".to_string(),
        'e' => "1110".to_string(),
        'f' => "1111".to_string()
    };
    let mut binary = String::new();
    for number in hex_address.chars(){
        if let Some(converted_value_of_single_character) =  my_map_of_hex_to_binary.get(&number){
        binary.push_str(converted_value_of_single_character);
    }
    else{
        return Err("Hex character not valid")
    }
}
    Ok(binary)
    
} 

struct TupleOfTagAndAddress<'a>{
    tag: &'a str,
    hex_address: &'a str,
    binary: String
}

//if first_byte != 32{ remember the below function should only run if this if statement is satisfied as 32 is equivalent to a space 

fn turn_line_sep_vector_into_tuple(line_of_vec: &str)-> Result<TupleOfTagAndAddress, &'static str>{
    let first_byte:i32 = line_of_vec.chars().next().unwrap() as i32;
    if first_byte == 32{
        let index_of_comma = line_of_vec.find(",").unwrap_or(line_of_vec.len()); //This tries to find the comma and if it fails it instead returns the length of the string 
        let line_of_vec_with_size_removed: &str = &line_of_vec[0..index_of_comma];
        let split_instruction: Vec<&str> = line_of_vec_with_size_removed.split_whitespace().collect();
        let tag_address = TupleOfTagAndAddress{
            tag: split_instruction[0],
            hex_address: split_instruction[1],
            binary: convert_from_hex_to_binary(split_instruction[1]).unwrap() 
        };
        Ok(tag_address)
    }
        else{
            return Err("First character is not a space and therefore it is ignored")
        }
    } 
    


if value_of_E == "1"{
    println!("This is a direct mapped cache")
}



let cache_bytes_size = calculate_cache_size(value_of_s, value_of_E, value_of_b);

println!("The size of the cache is {} bytes", cache_bytes_size);

let vec_of_trace_file = make_file_line_separated_vector(path_to_trace);



//Okay so we need to split the binary address into the tag bits, set bits and block bits and store the type of address it is alongside it. 

struct BinaryInTagSetBlockParts<>{
    type_of_mem_access: String,
    tag_bits: String,
    set_bits: String,
    block_bits: String
}

let mut vec_of_binary_split_memory_addresses: Vec<BinaryInTagSetBlockParts> = Vec::new();

for line in vec_of_trace_file{
    let cloned_line = line.to_string();
    let input_tuple = turn_line_sep_vector_into_tuple(&cloned_line).unwrap();
    let cloned_binary = input_tuple.binary.to_string();
    let length_of_binary = cloned_binary.len();
    let value_of_s_as_num: usize = value_of_s.parse().unwrap();
    let value_of_b_as_num: usize = value_of_b.parse().unwrap();

    let block_end = length_of_binary;
    let block_start = length_of_binary - value_of_b_as_num;
    let set_start = (length_of_binary - value_of_b_as_num) - value_of_s_as_num;
    let set_end = length_of_binary - value_of_b_as_num; 


    let mut binary_split_memory_address_to_input = BinaryInTagSetBlockParts{
            type_of_mem_access: input_tuple.tag.to_string(),
            tag_bits: cloned_binary[0..set_start].to_string(), 
            set_bits: cloned_binary[set_start..set_end].to_string(), 
            block_bits: cloned_binary[block_start..block_end].to_string()
        };

        //println!("The index of tag bits are 0..{:?}", set_start);
        //println!("The index of set bits are {:?}..{:?}", set_start, set_end);
        //println!("The index of block bits are {:?}..{:?}", block_start, block_end);
        //println!();
        //println!("{}", cloned_binary);
        //println!("Type of memory access {}, the tag bits {}, the set bits {}, the block bits {}", binary_split_memory_address_to_input.type_of_mem_access, binary_split_memory_address_to_input.tag_bits, binary_split_memory_address_to_input.set_bits, binary_split_memory_address_to_input.block_bits);
    
    //println!("Type of memory access {}, the tag bits {}, the set bits {}, the block bits {}", binary_split_memory_address_to_input.type_of_mem_access, binary_split_memory_address_to_input.tag_bits, binary_split_memory_address_to_input.set_bits, binary_split_memory_address_to_input.block_bits);
    //println!(); 
    //println!("Value of binary length {}, value of s {}, value of b {}", length_of_binary, value_of_s_as_num, value_of_b_as_num);

    vec_of_binary_split_memory_addresses.push(binary_split_memory_address_to_input)

}

for binary_split_memory_address in vec_of_binary_split_memory_addresses{
   
    println!("Type of memory access {}, the tag bits {}, the set bits {}, the block bits {}", binary_split_memory_address.type_of_mem_access, binary_split_memory_address.tag_bits, binary_split_memory_address.set_bits, binary_split_memory_address.block_bits);
    println!("The complete memory address from the CPU is: {}{}{}", binary_split_memory_address.tag_bits,binary_split_memory_address.set_bits,binary_split_memory_address.block_bits)
}

}