#[allow(unused)]
use core::num;
use std::vec;
#[allow(unused)]
use std::process::id;
#[allow(unused)]
use std::{alloc::System, env, fs::File, io::{self, BufRead, BufReader}, collections::HashMap};
#[macro_use]
extern crate maplit;
#[allow(unused)]

mod cache;
#[allow(unused)]

fn main() {

let args: Vec<String> = env::args().skip(1).collect(); //The skip here stops the executable binary being placed in the output


if args.len() < 8 || args.len() > 9 { //The max amount of arguments (if using the verbose and or helper flag) is 9 whereas the minimum is 8

    println!("Usage: [-hv] -s <s> -E <E> -b <b> -t <tracefile>"); //This just ensures the correct number of command line arguments are provided and provides the user with a usage guide if incorrect

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


#[allow(non_snake_case)]
fn calculate_cache_size(s: &String, E:&String, b:&String) -> u32{ //This dynamically calculates how many BYTES the combined cache is
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
        if !line_to_add.contains('I'){
            vec_of_lines.push(line_to_add);
        }
    }
    return vec_of_lines;
}


fn convert_from_hex_to_binary(hex_address: &str) -> Result<String, &'static str>{ //The function either returns a String or an error (as handled by &'static str) Just for future reference, the reason you chose a static lifetime is so the error message/string persists for the whole time the program executes as we might need to see at the last second that one of the strings failed to convert to binary
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
    size: &'a str,
    binary: String
}

fn turn_line_sep_vector_into_tuple(line_of_vec: &str)-> Result<TupleOfTagAndAddress, &'static str>{ //if first_byte != 32{ remember the below function should only run if this if statement is satisfied as 32 is equivalent to a space 
    let first_byte:i32 = line_of_vec.chars().next().unwrap() as i32;
    if first_byte == 32{
        let index_of_comma = line_of_vec.find(",").unwrap_or(line_of_vec.len()); //This tries to find the comma and if it fails it instead returns the length of the string 
        let index_of_size = index_of_comma + 1;
        let line_of_vec_with_size_removed: &str = &line_of_vec[0..index_of_comma];
        let split_instruction: Vec<&str> = line_of_vec_with_size_removed.split_whitespace().collect();
        let tag_address = TupleOfTagAndAddress{
            tag: split_instruction[0],
            hex_address: split_instruction[1],
            size: &line_of_vec[index_of_size..],
            binary: convert_from_hex_to_binary(split_instruction[1]).unwrap() 
        };
        Ok(tag_address)
    }
        else{
            return Err("First character is not a space and therefore it is ignored")
        }
    } 
    




let cache_bytes_size = calculate_cache_size(value_of_s, value_of_E, value_of_b);
let vec_of_trace_file = make_file_line_separated_vector(path_to_trace);


struct BinaryInTagSetBlockParts<>{//Okay so we need to split the binary address into the tag bits, set bits and block bits and store the type of address it is alongside it. 
    type_of_mem_access: String,
    tag_bits: String,
    set_bits: String,
    size: String
}

fn split_binary_address_into_type_t_s_and_b(vec_of_trace_file_input: Vec<String>, value_of_s: &String, value_of_b: &String)-> Result<Vec<BinaryInTagSetBlockParts>, &'static str>{
    if !vec_of_trace_file_input.is_empty(){
        let mut vec_of_binary_split_memory_addresses:Vec<BinaryInTagSetBlockParts>  = Vec::new();
        for line in vec_of_trace_file_input{
            let cloned_line = line.to_string();
            let input_tuple = turn_line_sep_vector_into_tuple(&cloned_line).unwrap();
            let cloned_binary = input_tuple.binary.to_string();
            let length_of_binary = cloned_binary.len();
            let value_of_s_as_num: usize = value_of_s.parse().unwrap();
            let value_of_b_as_num: usize = value_of_b.parse().unwrap();

            let block_bits_end_index = length_of_binary - value_of_b_as_num; 

            let mut binary_split_memory_address_to_input = BinaryInTagSetBlockParts{
                type_of_mem_access: input_tuple.tag.to_string(),
                tag_bits: cloned_binary[0..block_bits_end_index-value_of_s_as_num].to_string(), 
                set_bits: cloned_binary[block_bits_end_index-value_of_s_as_num..block_bits_end_index].to_string(), 
                size: input_tuple.size.to_string()
            };

            // println!("Original full binary: {}, Original hex address: {}, Tag bits: {}, Set bits: {}",&input_tuple.binary, &input_tuple.hex_address, &binary_split_memory_address_to_input.tag_bits, binary_split_memory_address_to_input.set_bits);

            vec_of_binary_split_memory_addresses.push(binary_split_memory_address_to_input);         
        }
        Ok(vec_of_binary_split_memory_addresses)
    }
    
    else{
        return Err("Vector is empty and therefore the for loop did not run")
    }

}

let mut vec_of_binary_split_memory_addresses = split_binary_address_into_type_t_s_and_b(vec_of_trace_file.clone(), value_of_s,value_of_b).unwrap();
    //Okay so to represent the cache sets, and the amount of cache lines within those sets I have decided to create a fixed size 2d array. 


    

let cache_sets: usize = 2_usize.pow(value_of_s.parse().unwrap()); //rows
let cache_lines: usize = value_of_E.to_string().parse().unwrap(); //columns add one as we need a column for the block stored within the cache 


    let mut my_cache = cache::ArrayRepresentationOfCache::new(value_of_s.parse().unwrap(), value_of_E.parse().unwrap(), cache_sets, cache_lines);

    // for instructions in vec_of_trace_file.clone(){
    //     println!("{}",instructions);
    // }
    
    // for binary in vec_of_binary_split_memory_addresses{
    //     let if_it_is_index_this_is_some = my_cache.is_set_in_cache(binary.set_bits.clone(), binary.tag_bits.clone());
    //     println!("These are the set bits and tag bits we are looking for {}, {}", binary.set_bits, binary.tag_bits);
    //     if if_it_is_index_this_is_some.is_some(){
    //         let index_of_vector_where_set_found = if_it_is_index_this_is_some.unwrap();
    //         my_cache.is_tag_in_cache(&binary.tag_bits.to_string(), &binary.set_bits.to_string(), index_of_vector_where_set_found);
    //     }
    //     println!();
    //     my_cache.print_array();
    // }

    // for binary in vec_of_binary_split_memory_addresses{
    //     println!("Tag bits in address: {}",binary.tag_bits);
    //     println!("Set bits in address: {}",binary.set_bits);
    //     my_cache.create_vector_with_blocks_after_tag_bits(binary.tag_bits, binary.set_bits);
    // }

    // for binary in vec_of_binary_split_memory_addresses{
        
    //     println!("Tag bits in address: {}",binary.tag_bits);
    //     println!("Set bits in address: {}",binary.set_bits);

    // }

    if value_of_s == "1" && value_of_E != "1"{
        my_cache.modify_two_d_array_to_be_correct_rows_and_correct_col_for_fully_associative();
    }

    for binary in vec_of_binary_split_memory_addresses{
        if value_of_E == "1"{ //This means direct mapped cache
            my_cache.create_two_d_array_with_index_if_dmc();
            my_cache.dmc_process(binary.set_bits.clone(), binary.tag_bits.clone(), binary.type_of_mem_access.clone());
        }
        if value_of_s == "1" && value_of_E != "1"{
           my_cache.insert_into_cache_if_fully_associative(binary.set_bits, binary.tag_bits, binary.type_of_mem_access);
        }
        // println!("This is what the cache looks like at this stage: ");
        // my_cache.print_array();

    }

    // my_cache.print_array();




    // my_cache.create_vector_with_blocks_after_tag_bits("00001111".to_string(), "1111".to_string());

    println!("hits:{} misses:{} evictions:{}", my_cache.cache_hits, my_cache.cache_misses, my_cache.cache_evictions);
    
    

    // my_cache.print_array();
}